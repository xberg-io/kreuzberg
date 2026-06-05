use std::path::Path;

fn main() {
    // Re-run whenever any Rust source changes.
    println!("cargo:rerun-if-changed=src");

    // Optional FRB codegen: regenerate flutter_rust_bridge artifacts when the
    // tool is on PATH. Missing tool is not fatal — committed generated sources
    // are checked in, and CI environments without FRB still build cleanly.
    match std::process::Command::new("flutter_rust_bridge_codegen")
        .args(["generate", "--config-file", "flutter_rust_bridge.yaml"])
        .status()
    {
        Ok(status) if status.success() => {
            // FRB v2.12+ emits `use` lists in an order rustfmt 2024 edition rewrites
            // (e.g. `{transform_result_dco, Lifetimeable, Lockable}` →
            // `{Lifetimeable, Lockable, transform_result_dco}`). Run rustfmt against
            // the generated file so committed output is fmt-clean and `cargo fmt --check`
            // stays green in CI.
            match std::process::Command::new("rustfmt")
                .args(["--edition", "2024", "src/frb_generated.rs"])
                .status()
            {
                Ok(s) if s.success() => {}
                Ok(s) => println!("cargo:warning=rustfmt on src/frb_generated.rs exited {s}"),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    println!(
                        "cargo:warning=rustfmt not on PATH — skipping post-FRB format. Install rustfmt via rustup to keep generated bridge sources fmt-clean."
                    );
                }
                Err(err) => println!("cargo:warning=failed to spawn rustfmt: {err}"),
            }

            // Patch the generated Dart entrypoint so the published package resolves
            // its native library from its own installed location.
            patch_published_loader();

            // Rewrite FRB-generated handler.executeSync/handler.executeNormal calls
            // into direct handler invocations. FRB 2.x emits these calls assuming
            // `handler` is a BaseHandler field, but in service-API methods `handler`
            // is a user-supplied function parameter (FutureOr<R> Function(T)) which
            // does not expose those methods, so the generated Dart fails to compile.
            // The rewrite is idempotent (marker-gated) and runs after every FRB
            // invocation — including the rebuild that fires during `dart pub get`
            // in e2e flows, which is when this otherwise reverts.
            fix_handler_executor_calls();
        }
        Ok(status) => panic!("flutter_rust_bridge_codegen generate failed (exit code: {status})"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!(
                "cargo:warning=flutter_rust_bridge_codegen not on PATH — skipping codegen. Install via `cargo install flutter_rust_bridge_codegen --locked` to regenerate FRB artifacts at build time."
            );
        }
        Err(err) => panic!("failed to spawn flutter_rust_bridge_codegen: {err}"),
    }
}

const FRB_GENERATED_DART: &str = "../lib/src/kreuzberg_bridge_generated/frb_generated.dart";
const FRB_HANDLER_EXECUTOR_CALLS_MARKER: &str = "handler.executeSync";
const LOADER_MARKER: &str = "_alefResolveExternalLibrary";
const FRB_INIT_PROLOGUE: &str = "  /// Initialize flutter_rust_bridge\n  static Future<void> init({\n    RustLibApi? api,\n    BaseHandler? handler,\n    ExternalLibrary? externalLibrary,\n    bool forceSameCodegenVersion = true,\n  }) async {\n";
const FRB_INIT_REPLACEMENT: &str = r#"  /// Resolve the prebuilt native library from this package's own installed
  /// location so the load works from any working directory and under hardened
  /// runtimes. Returns `null` to defer to flutter_rust_bridge's default loader.
  ///
  /// Published pub.dev packages stage natives under `lib/src/native/<rid>/`
  /// (e.g. `macos-arm64`, `linux-x64`). For local FRB-dev builds the dylib is
  /// emitted into `lib/src/kreuzberg_bridge_generated/`; that
  /// path is searched as a fallback.
  static Future<ExternalLibrary?> _alefResolveExternalLibrary() async {
    try {
      final packageRoot =
          await Isolate.resolvePackageUri(Uri.parse('package:kreuzberg/kreuzberg.dart'));
      if (packageRoot == null) return null;
      final libNames = _alefHostLibNames();
      final searchDirs = <Uri>[
        if (_alefHostRid() != null) packageRoot.resolve('src/native/${_alefHostRid()}/'),
        packageRoot.resolve('src/kreuzberg_bridge_generated/'),
      ];
      for (final dir in searchDirs) {
        for (final name in libNames) {
          final libPath = dir.resolve(name).toFilePath();
          if (File(libPath).existsSync()) {
            return ExternalLibrary.open(libPath);
          }
        }
      }
    } catch (_) {
      // Fall through to the default loader on any resolution failure.
    }
    return null;
  }

  /// Map the host platform to the pub.dev native staging RID. Returns `null`
  /// for unrecognized host triples so the FRB-dev fallback path runs instead.
  static String? _alefHostRid() {
    final abi = Abi.current();
    if (abi == Abi.macosArm64) return 'macos-arm64';
    if (abi == Abi.macosX64) return 'macos-x64';
    if (abi == Abi.linuxArm64) return 'linux-arm64';
    if (abi == Abi.linuxX64) return 'linux-x64';
    if (abi == Abi.windowsArm64) return 'windows-arm64';
    if (abi == Abi.windowsX64) return 'windows-x64';
    return null;
  }

  static List<String> _alefHostLibNames() {
    // The Dart-binding Rust crate is `{stem}-dart` (per the cargo manifest
    // template), which produces a cdylib named `lib{stem}_dart.{ext}` on Unix
    // and `{stem}_dart.dll` on Windows.
    if (Platform.isMacOS) return const ['libkreuzberg_dart.dylib'];
    if (Platform.isWindows) return const ['kreuzberg_dart.dll'];
    return const ['libkreuzberg_dart.so'];
  }

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
    bool forceSameCodegenVersion = true,
  }) async {
    externalLibrary ??= await _alefResolveExternalLibrary();
"#;

/// Inject the published-package native-library loader into `frb_generated.dart`.
/// Idempotent: a no-op when the marker is already present or the FRB entrypoint
/// signature is absent.
fn patch_published_loader() {
    let path = Path::new(FRB_GENERATED_DART);
    let Ok(source) = std::fs::read_to_string(path) else {
        println!(
            "cargo:warning=published-loader patch skipped: {} not found",
            FRB_GENERATED_DART
        );
        return;
    };
    if source.contains(LOADER_MARKER) {
        return;
    }
    if !source.contains(FRB_INIT_PROLOGUE) {
        println!("cargo:warning=published-loader patch skipped: FRB init prologue not found");
        return;
    }

    let mut patched = source.replacen(FRB_INIT_PROLOGUE, FRB_INIT_REPLACEMENT, 1);

    // Ensure the helper's `File`/`Isolate`/`Abi` dependencies are imported.
    for (probe, line) in [
        ("import 'dart:io';", "import 'dart:io';\n"),
        ("import 'dart:isolate';", "import 'dart:isolate';\n"),
        ("import 'dart:ffi';", "import 'dart:ffi';\n"),
    ] {
        if patched.contains(probe) {
            continue;
        }
        if let Some(pos) = patched.find("\nimport ") {
            patched.insert_str(pos + 1, line);
        } else {
            patched.insert_str(0, line);
        }
    }

    if patched != source {
        if let Err(err) = std::fs::write(path, &patched) {
            println!("cargo:warning=failed to write published-loader patch: {err}");
            return;
        }
        match std::process::Command::new("dart")
            .args(["format", FRB_GENERATED_DART])
            .status()
        {
            Ok(s) if s.success() => {}
            Ok(s) => println!("cargo:warning=dart format on {} exited {}", FRB_GENERATED_DART, s),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                println!(
                    "cargo:warning=dart not on PATH — skipping post-patch format. Install Dart SDK to keep generated FRB Dart sources fmt-clean."
                );
            }
            Err(err) => println!("cargo:warning=failed to spawn dart format: {err}"),
        }
    }
}

/// Rewrite the FRB-generated `handler.executeSync(...)` and
/// `handler.executeNormal(...)` calls on callback function parameters.
///
/// FRB 2.x emits `handler.executeSync(SyncTask(...))` inside service-API
/// methods that take a user-supplied `handler` callback parameter; those
/// methods don't exist on plain function types. This rewrite strips the
/// erroneous method calls, calling the handler directly as a function.
///
/// Also fixes FRB 2.x bug where class declarations incorrectly include the
/// `async` keyword (e.g. `class RustLibApiImpl implements RustLibApi async ...`).
/// The `async` keyword is only valid on functions, not class declarations.
///
/// Idempotent: when the broken patterns are absent the function is a no-op.
fn fix_handler_executor_calls() {
    let path = Path::new(FRB_GENERATED_DART);
    let Ok(source) = std::fs::read_to_string(path) else {
        return;
    };

    if !source.contains(FRB_HANDLER_EXECUTOR_CALLS_MARKER) {
        return;
    }

    let mut fixed = source
        .replace("handler.executeSync(", "await handler(")
        .replace("handler.executeNormal(", "await handler(");

    // Collapse `return await` + `await handler(` → `return await handler(`.
    fixed = fixed.replace("await await handler", "await handler");

    // Fix FRB 2.x bug where class declarations have invalid `async` keyword.
    // class RustLibApiImpl implements RustLibApi async { ... becomes class RustLibApiImpl implements RustLibApi { ...
    fixed = fixed.replace(" implements RustLibApi async {", " implements RustLibApi {");

    if fixed != source
        && let Err(err) = std::fs::write(path, &fixed)
    {
        println!(
            "cargo:warning=failed to fix handler executor calls in {}: {err}",
            FRB_GENERATED_DART
        );
    }
}

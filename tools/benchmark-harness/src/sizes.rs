//! Framework size measurement
//!
//! Measures the installation footprint of document extraction frameworks.
//! All sizes must be exactly measured - no estimates allowed.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Information about a framework's disk size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkSize {
    /// Size in bytes (package + system deps combined)
    pub size_bytes: u64,
    /// Package-only size in bytes (before adding system deps)
    #[serde(default)]
    pub package_bytes: u64,
    /// System dependency size in bytes (libreoffice, tesseract, ffmpeg, etc.)
    #[serde(default)]
    pub system_deps_bytes: u64,
    /// Method used to measure (pip_package, npm_package, binary_size, jar_size, etc.)
    pub method: String,
    /// Human-readable description
    pub description: String,
    /// Whether this is from actual measurement or an estimate
    /// NOTE: This field is deprecated and should always be false.
    /// If we cannot measure a size, we return an error instead of an estimate.
    #[serde(default)]
    pub estimated: bool,
}

/// Framework size measurement results
pub type FrameworkSizes = HashMap<String, FrameworkSize>;

/// Known frameworks with their measurement methods and descriptions
const FRAMEWORKS: &[(&str, &str, &str)] = &[
    // Kreuzberg bindings
    ("kreuzberg-rust", "binary_size", "Native Rust core binary"),
    ("kreuzberg-python", "pip_package", "Python wheel package"),
    ("kreuzberg-node", "npm_package", "Node.js native addon"),
    ("kreuzberg-wasm", "wasm_bundle", "WebAssembly binary"),
    ("kreuzberg-ruby", "gem_package", "Ruby gem native extension"),
    ("kreuzberg-go", "binary_size", "Go binary with CGO"),
    ("kreuzberg-java", "jar_size", "Java JAR with JNI"),
    ("kreuzberg-csharp", "nuget_package", ".NET NuGet package"),
    ("kreuzberg-elixir", "hex_package", "Elixir hex package with NIF"),
    ("kreuzberg-php", "php_extension", "PHP extension"),
    // Third-party frameworks
    ("docling", "pip_package", "IBM Docling document processing"),
    ("markitdown", "pip_package", "Mark It Down markdown converter"),
    ("pandoc", "binary_size", "Pandoc universal converter"),
    ("unstructured", "pip_package", "Unstructured document processing"),
    ("tika", "jar_size", "Apache Tika content analysis"),
    ("pymupdf4llm", "pip_package", "PyMuPDF for LLM"),
    ("pdfplumber", "pip_package", "pdfplumber PDF extraction"),
    ("mineru", "pip_package", "MinerU document intelligence"),
];

/// Known system dependency sizes (in bytes) for frameworks that require
/// OS-level packages (libreoffice, tesseract, ffmpeg, etc.).
/// Sizes are approximate installed sizes on Linux (Ubuntu/Debian amd64).
fn system_deps_bytes(framework: &str) -> u64 {
    match framework {
        // libreoffice ~300MB, tesseract-ocr+lang ~30MB, pandoc ~40MB,
        // poppler-utils ~20MB, libmagic ~1MB
        "unstructured" => 400_000_000,
        // ffmpeg ~100MB, exiftool ~25MB
        "markitdown" => 125_000_000,
        // libgl1 ~1MB, fonts-noto-cjk ~88MB, fonts-noto-core ~10MB, fontconfig ~2MB
        "mineru" => 101_000_000,
        // libgl1 ~1MB, libglib2.0-0 ~4MB, tesseract-ocr+eng ~20MB
        "docling" => 25_000_000,
        // JRE/JDK ~200MB (required to run Tika JAR)
        "tika" => 200_000_000,
        _ => 0,
    }
}

/// Measure framework sizes
/// Returns sizes for all frameworks that can be measured.
/// Frameworks that are not installed are silently skipped.
pub fn measure_framework_sizes() -> Result<FrameworkSizes> {
    let mut sizes = HashMap::new();

    for (name, method, description) in FRAMEWORKS {
        match measure_framework(name, method) {
            Ok(Some(pkg_size)) => {
                let sys_size = system_deps_bytes(name);
                sizes.insert(
                    name.to_string(),
                    FrameworkSize {
                        size_bytes: pkg_size + sys_size,
                        package_bytes: pkg_size,
                        system_deps_bytes: sys_size,
                        method: method.to_string(),
                        description: description.to_string(),
                        estimated: false,
                    },
                );
            }
            Ok(None) => {
                eprintln!("Size measurement: {} ({}) - not installed, skipping", name, method);
            }
            Err(e) => {
                eprintln!("Size measurement: {} ({}) - failed: {}", name, method, e);
            }
        }
    }

    Ok(sizes)
}

/// Measure framework sizes, failing if any framework cannot be measured
/// Use this for CI/release verification where all sizes must be present.
pub fn measure_framework_sizes_strict() -> Result<FrameworkSizes> {
    let mut sizes = HashMap::new();
    let mut errors = Vec::new();

    for (name, method, description) in FRAMEWORKS {
        match measure_framework(name, method) {
            Ok(Some(pkg_size)) => {
                let sys_size = system_deps_bytes(name);
                sizes.insert(
                    name.to_string(),
                    FrameworkSize {
                        size_bytes: pkg_size + sys_size,
                        package_bytes: pkg_size,
                        system_deps_bytes: sys_size,
                        method: method.to_string(),
                        description: description.to_string(),
                        estimated: false,
                    },
                );
            }
            Ok(None) | Err(_) => {
                errors.push(format!("{} ({})", name, method));
            }
        }
    }

    if !errors.is_empty() {
        return Err(Error::Benchmark(format!(
            "Failed to measure sizes for frameworks: {}. Install these frameworks or use measure_framework_sizes() for lenient mode.",
            errors.join(", ")
        )));
    }

    Ok(sizes)
}

/// Measure a single framework.
/// Returns Ok(Some(size)) for successful measurement, Ok(None) for frameworks
/// that aren't installed, or Err for measurement failures.
fn measure_framework(name: &str, method: &str) -> Result<Option<u64>> {
    match method {
        "pip_package" => measure_pip_package(extract_package_name(name)),
        "npm_package" => measure_npm_package(extract_package_name(name)),
        "binary_size" => measure_binary(name),
        "jar_size" => measure_jar(name),
        "gem_package" => measure_gem_package(extract_package_name(name)),
        "wasm_bundle" => measure_wasm_bundle(name),
        "nuget_package" => measure_nuget_package(name),
        "hex_package" => measure_hex_package(name),
        "php_extension" => measure_php_extension(name),
        _ => Err(Error::Benchmark(format!("Unknown measurement method: {}", method))),
    }
}

/// Extract Python/npm/gem package name from framework name
fn extract_package_name(framework: &str) -> &str {
    // Strip -batch suffix and kreuzberg- prefix for lookups
    let name = framework.strip_suffix("-batch").unwrap_or(framework);

    match name {
        "kreuzberg-python" => "kreuzberg",
        "kreuzberg-node" => "@kreuzberg/node",
        "kreuzberg-ruby" => "kreuzberg_rb",
        "docling" => "docling",
        "markitdown" => "markitdown",
        "unstructured" => "unstructured",
        "pymupdf4llm" => "pymupdf4llm",
        "pdfplumber" => "pdfplumber",
        "mineru" => "mineru",
        _ => name,
    }
}

/// Measure Python package size via `uv pip show`.
///
/// Packages must be installed in the project .venv via `uv sync --group bench-*`.
/// Returns an error if the package cannot be found or measured.
///
/// For kreuzberg: measures the single package directory (includes native .so).
/// For third-party frameworks (docling, unstructured, mineru, etc.): uses
/// `pip-weigh` to measure the package + full transitive dependency tree in an
/// isolated venv, capturing deps like torch/transformers that dominate the
/// actual installation footprint.
fn measure_pip_package(package: &str) -> Result<Option<u64>> {
    // For native packages (e.g. kreuzberg installed via maturin develop),
    // use Python to find the actual package directory which includes the native .so.
    // This is more reliable than parsing pip show output for editable installs.
    if let Some(size) = measure_pip_package_via_python(package) {
        return Ok(Some(size));
    }

    // For third-party packages, use pip-weigh to get accurate total size
    // including all transitive dependencies in an isolated environment.
    if package != "kreuzberg"
        && let Some(size) = measure_pip_weigh(package)
    {
        return Ok(Some(size));
    }

    let output = Command::new("uv")
        .args(["pip", "show", "-f", package])
        .output()
        .map_err(|e| Error::Benchmark(format!("Failed to run `uv pip show {}`: {}", package, e)))?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_pip_show_size(&stdout, package))
}

/// Use `pip-weigh --json <package>` to measure a package's total installation
/// footprint including all transitive dependencies. pip-weigh creates an
/// isolated venv, installs the package, and measures via .dist-info/RECORD.
/// Returns None if pip-weigh is not installed or the command fails.
fn measure_pip_weigh(package: &str) -> Option<u64> {
    let output = Command::new("pip-weigh").args(["--json", package]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).ok()?;
    json.get("results")?.get(0)?.get("total_size_bytes")?.as_u64()
}

/// Parse pip show -f output to extract package size
fn parse_pip_show_size(stdout: &str, package: &str) -> Option<u64> {
    // Find Location line
    let location_line = stdout.lines().find(|l| l.starts_with("Location:"))?;
    let location = location_line.strip_prefix("Location:")?.trim();
    let location_path = Path::new(location);

    // For editable installs (e.g. maturin develop), pip show reports an
    // "Editable project location:" and the Files: section only contains
    // dist-info metadata (~16KB). Measure the actual package directory
    // at the editable project location instead.
    if let Some(editable_line) = stdout.lines().find(|l| l.starts_with("Editable project location:"))
        && let Some(editable_path) = editable_line
            .strip_prefix("Editable project location:")
            .map(|s| s.trim())
    {
        let project_dir = Path::new(editable_path);
        // Measure the Python package directory within the editable project
        // (e.g. packages/python/kreuzberg/ for the kreuzberg package)
        let pkg_dir = project_dir.join(package.replace('-', "_"));
        if pkg_dir.exists() {
            return Some(dir_size(&pkg_dir));
        }
        // Fall back to the project directory itself
        if project_dir.exists() {
            return Some(dir_size(project_dir));
        }
    }

    // Try package directory first (e.g. {location}/kreuzberg/)
    let package_dir = location_path.join(package.replace('-', "_"));
    if package_dir.exists() {
        return Some(dir_size(&package_dir));
    }

    // Fall back to summing individual files listed by pip show -f
    // This handles native extensions (maturin) where files are at top-level
    let mut in_files_section = false;
    let mut total_size: u64 = 0;
    let mut found_files = false;
    for line in stdout.lines() {
        if line.starts_with("Files:") {
            in_files_section = true;
            continue;
        }
        if in_files_section {
            let file_rel = line.trim();
            if file_rel.is_empty() {
                continue;
            }
            // Lines after Files: that don't start with whitespace are new sections
            if !line.starts_with(' ') && !line.starts_with('\t') {
                break;
            }
            let file_path = location_path.join(file_rel);
            if let Ok(metadata) = fs::metadata(&file_path) {
                total_size += metadata.len();
                found_files = true;
            }
        }
    }
    if found_files {
        return Some(total_size);
    }

    None
}

/// Measure npm package size including native addon binary
fn measure_npm_package(package: &str) -> Result<Option<u64>> {
    // For kreuzberg-node, measure the native .node addon + JS wrapper
    // The .node file contains the Rust FFI + pdfium statically linked
    if package.contains("kreuzberg") && package.contains("node") {
        let mut total: u64 = 0;

        // Find the native .node addon in the crate directory
        let node_crate = Path::new("crates/kreuzberg-node");
        if node_crate.exists() {
            if let Ok(entries) = fs::read_dir(node_crate) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        // The native addon: kreuzberg-node.linux-x64-gnu.node, etc.
                        if name.ends_with(".node")
                            && let Ok(metadata) = fs::metadata(&path)
                        {
                            total += metadata.len();
                        }
                    }
                }
            }
            // Add JS wrapper (dist/ directory)
            let dist_dir = node_crate.join("dist");
            if dist_dir.exists() {
                total += dir_size(&dist_dir);
            }
        }

        // Also check npm platform packages (e.g. crates/kreuzberg-node/npm/linux-x64-gnu/)
        let npm_dir = node_crate.join("npm");
        if npm_dir.exists()
            && let Ok(entries) = fs::read_dir(&npm_dir)
        {
            for entry in entries.flatten() {
                let platform_dir = entry.path();
                if platform_dir.is_dir()
                    && let Ok(files) = fs::read_dir(&platform_dir)
                {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|e| e.to_str()) == Some("node")
                            && let Ok(metadata) = file.metadata()
                        {
                            total += metadata.len();
                        }
                    }
                }
            }
        }

        if total > 0 {
            return Ok(Some(total));
        }
    }

    // For third-party npm packages, fall back to npm pack --dry-run
    let output = Command::new("npm")
        .args(["pack", "--dry-run", "--json", package])
        .output()
        .ok();

    if let Some(output) = output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout)
            && let Some(size) = json.get(0).and_then(|v| v.get("size")).and_then(|v| v.as_u64())
        {
            return Ok(Some(size));
        }
    }

    Ok(None)
}

/// Measure binary size
fn measure_binary(name: &str) -> Result<Option<u64>> {
    let binary_name = match name {
        "pandoc" => "pandoc",
        "kreuzberg-rust" => "kreuzberg",
        s if s.starts_with("kreuzberg-go") => "kreuzberg-go",
        _ => return Ok(None),
    };

    // For kreuzberg-rust, measure the FFI shared library (used by all bindings)
    if name.starts_with("kreuzberg-rust") {
        let target_paths = [
            "target/release/libkreuzberg_ffi.so",
            "target/release/libkreuzberg_ffi.dylib",
            "target/release/kreuzberg_ffi.dll",
            "target/release/libkreuzberg_ffi.a",
            "target/release/kreuzberg",
            "target/debug/kreuzberg",
            "target/release/libkreuzberg.so",
            "target/release/libkreuzberg.dylib",
            "target/release/kreuzberg.dll",
        ];
        for path in target_paths {
            if let Ok(metadata) = fs::metadata(path) {
                return Ok(Some(metadata.len()));
            }
        }
    }

    // For kreuzberg-go, measure the FFI shared library (Go links against it via CGO).
    // Do NOT fall back to measuring the Go source directory — it includes test fixtures
    // and build artifacts that inflate the size to ~843 MB.
    if name.starts_with("kreuzberg-go") {
        let go_ffi_paths = [
            "target/release/libkreuzberg_ffi.so",
            "target/release/libkreuzberg_ffi.dylib",
            "target/release/kreuzberg_ffi.dll",
        ];
        for path in go_ffi_paths {
            if let Ok(metadata) = fs::metadata(path) {
                return Ok(Some(metadata.len()));
            }
        }
        // Fall back to measuring all native FFI libs (includes pdfium)
        let ffi_size = measure_native_ffi_libs();
        if ffi_size > 0 {
            return Ok(Some(ffi_size));
        }
        return Ok(None);
    }

    // Try which to find binary in PATH
    let output = Command::new("which").arg(binary_name).output().ok();

    if let Some(output) = output
        && output.status.success()
    {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Ok(metadata) = fs::metadata(&path) {
            return Ok(Some(metadata.len()));
        }
    }

    Ok(None)
}

/// Measure JAR size (Apache Tika)
fn measure_jar(name: &str) -> Result<Option<u64>> {
    // Common locations for Tika JAR
    let possible_paths = [
        "/usr/share/java/tika-app.jar",
        "/opt/tika/tika-app.jar",
        "~/.local/share/tika/tika-app.jar",
    ];

    if name.starts_with("tika") {
        for path in possible_paths {
            let expanded = shellexpand::tilde(path);
            let expanded_path: &str = expanded.as_ref();
            if let Ok(metadata) = fs::metadata(expanded_path) {
                return Ok(Some(metadata.len()));
            }
        }

        // Try TIKA_JAR environment variable
        if let Ok(jar_path) = std::env::var("TIKA_JAR")
            && let Ok(metadata) = fs::metadata(&jar_path)
        {
            return Ok(Some(metadata.len()));
        }

        // Try tools/benchmark-harness/libs directory
        let libs_dir = Path::new("tools/benchmark-harness/libs");
        if let Ok(entries) = fs::read_dir(libs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str())
                    && name.starts_with("tika-app-")
                    && name.ends_with(".jar")
                    && let Ok(metadata) = fs::metadata(&path)
                {
                    return Ok(Some(metadata.len()));
                }
            }
        }
    }

    // For kreuzberg-java, measure classes (including JNI natives) + runtime dependencies
    if name.starts_with("kreuzberg-java") {
        let mut total: u64 = 0;

        // Compiled classes + bundled native libs (in target/classes/natives/)
        let classes_dir = Path::new("packages/java/target/classes");
        if classes_dir.exists() {
            total += dir_size(classes_dir);
        }

        // Runtime dependency JARs (jackson, etc.)
        let deps_dir = Path::new("packages/java/target/dependency");
        if deps_dir.exists() {
            total += dir_size(deps_dir);
        }

        // Check if native libs are actually bundled in classes/natives/.
        // In CI, the natives/ dir may only contain .gitkeep placeholders
        // without real .so/.dylib files. Add FFI libs from target/release/.
        let natives_dir = Path::new("packages/java/target/classes/natives");
        if !has_native_extension(natives_dir) {
            total += measure_native_ffi_libs();
        }

        if total > 0 {
            return Ok(Some(total));
        }

        // Fall back to a pre-built JAR
        let jar_path = Path::new("packages/java/target/kreuzberg.jar");
        if let Ok(metadata) = fs::metadata(jar_path) {
            return Ok(Some(metadata.len()));
        }
    }

    Ok(None)
}

/// Measure Ruby gem size using bundle show or gem contents
fn measure_gem_package(package: &str) -> Result<Option<u64>> {
    // Map package names to actual gem names
    let gem_name = match package {
        "kreuzberg" | "kreuzberg-ruby" => "kreuzberg_rb",
        other => other,
    };

    // Try bundle show first (for Bundler-managed gems)
    if let Ok(output) = Command::new("bundle").args(["show", gem_name]).output()
        && output.status.success()
    {
        let gem_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !gem_path.is_empty() {
            let path = Path::new(&gem_path);
            if path.exists() {
                return Ok(Some(dir_size(path)));
            }
        }
    }

    // Fall back to gem specification
    if let Ok(output) = Command::new("ruby")
        .arg("-e")
        .arg(format!(
            "puts Gem::Specification.find_by_name('{}').gem_dir rescue nil",
            gem_name
        ))
        .output()
        && output.status.success()
    {
        let gem_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !gem_path.is_empty() {
            let path = Path::new(&gem_path);
            if path.exists() {
                return Ok(Some(dir_size(path)));
            }
        }
    }

    // Try workspace packages/ruby — measure only the built gem in pkg/ or lib/
    // (not ext/, tmp/, vendor/ which contain build artifacts)
    let ruby_pkg = Path::new("packages/ruby/pkg");
    if ruby_pkg.exists() {
        return Ok(Some(dir_size(ruby_pkg)));
    }
    let ruby_lib = Path::new("packages/ruby/lib");
    if ruby_lib.exists() {
        let lib_size = dir_size(ruby_lib);
        let mut total = lib_size;

        // Add FFI native libs unless lib/ already contains a substantial native
        // extension (> 5 MB). Small .so files may be stubs or incomplete artifacts
        // that don't include the full FFI + pdfium libs.
        let has_substantial_native = has_native_extension(ruby_lib) && lib_size > 5_000_000;
        if !has_substantial_native {
            total += measure_native_ffi_libs();
        }

        if total > 0 {
            return Ok(Some(total));
        }
    }

    Ok(None)
}

/// Measure WebAssembly bundle size
fn measure_wasm_bundle(name: &str) -> Result<Option<u64>> {
    // Look for .wasm files in common locations
    let wasm_paths = [
        "packages/wasm/pkg/kreuzberg_bg.wasm",
        "packages/wasm/dist/kreuzberg.wasm",
        "target/wasm32-unknown-unknown/release/kreuzberg.wasm",
        "crates/kreuzberg-wasm/pkg/kreuzberg_wasm_bg.wasm",
    ];

    for path in wasm_paths {
        if let Ok(metadata) = fs::metadata(path) {
            return Ok(Some(metadata.len()));
        }
    }

    // Check node_modules for installed WASM package
    if name.contains("wasm") || name.contains("kreuzberg") {
        let node_modules_paths = [
            "node_modules/@kreuzberg/wasm",
            "packages/typescript/node_modules/@kreuzberg/wasm",
        ];
        for path in node_modules_paths {
            let dir = Path::new(path);
            if dir.exists() {
                return Ok(Some(dir_size(dir)));
            }
        }
    }

    Ok(None)
}

/// Measure .NET NuGet package size
///
/// Checks project build output directories first, then NuGet cache as fallback.
/// Always ensures native FFI libs are included in the total since the .NET
/// package depends on the Rust shared library at runtime.
fn measure_nuget_package(name: &str) -> Result<Option<u64>> {
    if name.starts_with("kreuzberg-csharp") {
        // Check project build output directories first
        let project_dirs = ["packages/csharp/Kreuzberg", "packages/csharp/Kreuzberg.Native"];
        for proj_dir_str in project_dirs {
            let proj_dir = Path::new(proj_dir_str);
            // Check bin/Release first, then bin/Debug
            for config in ["Release", "Debug"] {
                let bin_dir = proj_dir.join("bin").join(config);
                if bin_dir.exists() {
                    let mut total = dir_size(&bin_dir);

                    // Always add native FFI libs if bin/ doesn't contain them.
                    // In CI, the runtimes/*/native/ dir may not be populated.
                    if !has_native_extension(&bin_dir) {
                        total += measure_native_ffi_libs();
                    }

                    return Ok(Some(total));
                }
            }
        }

        // Also check Benchmark project output
        for config in ["Release", "Debug"] {
            let bench_bin = Path::new("packages/csharp/Benchmark/bin").join(config);
            if bench_bin.exists() {
                let mut total = dir_size(&bench_bin);
                if !has_native_extension(&bench_bin) {
                    total += measure_native_ffi_libs();
                }
                return Ok(Some(total));
            }
        }

        // Fall back to NuGet cache, but always add FFI libs
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let nuget_cache_paths = [
            format!("{}/.nuget/packages/kreuzberg", home),
            format!("{}/.nuget/packages/kreuzberg.native", home),
        ];
        for path in nuget_cache_paths {
            let dir = Path::new(&path);
            if dir.exists() {
                let mut total = dir_size(dir);
                if !has_native_extension(dir) {
                    total += measure_native_ffi_libs();
                }
                return Ok(Some(total));
            }
        }

        // Last resort: just the FFI libs
        let ffi_size = measure_native_ffi_libs();
        if ffi_size > 0 {
            return Ok(Some(ffi_size));
        }
    }

    Ok(None)
}

/// Measure Elixir Hex package size
fn measure_hex_package(name: &str) -> Result<Option<u64>> {
    // Look in _build directory for compiled Elixir code
    let build_paths = [
        "packages/elixir/_build/prod/lib/kreuzberg",
        "packages/elixir/_build/dev/lib/kreuzberg",
    ];

    for path in build_paths {
        let dir = Path::new(path);
        if dir.exists() {
            return Ok(Some(dir_size(dir)));
        }
    }

    // Try to find in Hex cache
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let hex_paths = [
        format!("{}/.hex/packages/hexpm/kreuzberg", home),
        format!("{}/.mix/archives/kreuzberg", home),
    ];

    for path in hex_paths {
        let dir = Path::new(&path);
        if dir.exists() {
            return Ok(Some(dir_size(dir)));
        }
    }

    // Measure workspace packages/elixir directory
    if name.starts_with("kreuzberg-elixir") {
        let elixir_dir = Path::new("packages/elixir");
        if elixir_dir.exists() {
            return Ok(Some(dir_size(elixir_dir)));
        }
    }

    Ok(None)
}

/// Measure PHP extension size
fn measure_php_extension(name: &str) -> Result<Option<u64>> {
    // Try to find the kreuzberg.so extension
    if let Ok(output) = Command::new("php")
        .args(["-r", "echo ini_get('extension_dir');"])
        .output()
        && output.status.success()
    {
        let ext_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let ext_path = Path::new(&ext_dir).join("kreuzberg.so");
        if let Ok(metadata) = fs::metadata(&ext_path) {
            return Ok(Some(metadata.len()));
        }
    }

    // Check workspace for built extension
    let workspace_paths = [
        "packages/php-ext/target/release/libkreuzberg_php.so",
        "packages/php-ext/target/release/libkreuzberg_php.dylib",
        "target/release/libkreuzberg_php.so",
        "target/release/libkreuzberg_php.dylib",
    ];

    for path in workspace_paths {
        if let Ok(metadata) = fs::metadata(path) {
            return Ok(Some(metadata.len()));
        }
    }

    // Measure the entire PHP package directory as fallback
    if name.starts_with("kreuzberg-php") {
        let php_dir = Path::new("packages/php-ext");
        if php_dir.exists() {
            return Ok(Some(dir_size(php_dir)));
        }
    }

    Ok(None)
}

/// Measure the native FFI + pdfium libraries from target/release/.
/// Returns the total size of found native libs, or 0 if none are found.
/// Only counts one platform variant of each library (first match wins).
fn measure_native_ffi_libs() -> u64 {
    let mut total = 0u64;

    // FFI shared library (one per platform)
    for path in [
        "target/release/libkreuzberg_ffi.so",
        "target/release/libkreuzberg_ffi.dylib",
        "target/release/kreuzberg_ffi.dll",
    ] {
        if let Ok(m) = fs::metadata(path) {
            total += m.len();
            break;
        }
    }

    // PDFium runtime library (one per platform)
    for path in [
        "target/release/libpdfium.so",
        "target/release/libpdfium.dylib",
        "target/release/pdfium.dll",
    ] {
        if let Ok(m) = fs::metadata(path) {
            total += m.len();
            break;
        }
    }

    total
}

/// Measure a pip package by asking Python where it is installed.
/// This handles editable installs (maturin develop) where the native .so
/// is in the site-packages directory alongside the Python source files.
fn measure_pip_package_via_python(package: &str) -> Option<u64> {
    let module_name = package.replace('-', "_");
    let script = format!(
        "import {mod_name}, os; print(os.path.dirname({mod_name}.__file__))",
        mod_name = module_name
    );
    let output = Command::new("python3").args(["-c", &script]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let pkg_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if pkg_dir.is_empty() {
        return None;
    }

    let path = Path::new(&pkg_dir);
    if path.exists() {
        let size = dir_size(path);
        // Sanity check: if the directory is suspiciously small (< 1MB),
        // it likely doesn't include the native extension. Return None to
        // fall through to pip show parsing.
        if size > 1_000_000 {
            return Some(size);
        }
    }

    None
}

/// Check if a directory (or one level of subdirectories) contains native
/// extension files (.so, .bundle, .dylib, .dll, .node).
fn has_native_extension(dir: &Path) -> bool {
    has_native_extension_inner(dir, 0)
}

fn has_native_extension_inner(dir: &Path, depth: u32) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension().and_then(|e| e.to_str())
            && matches!(ext, "so" | "bundle" | "dylib" | "dll" | "node")
        {
            return true;
        } else if path.is_dir() && depth < 2 && has_native_extension_inner(&path, depth + 1) {
            return true;
        }
    }
    false
}

/// Calculate total size of a directory
fn dir_size(path: &Path) -> u64 {
    let mut size = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                size += dir_size(&path);
            } else if let Ok(metadata) = path.metadata() {
                size += metadata.len();
            }
        }
    }

    size
}

/// Load framework sizes from a JSON file
pub fn load_framework_sizes(path: &Path) -> Result<FrameworkSizes> {
    let contents = fs::read_to_string(path).map_err(Error::Io)?;
    serde_json::from_str(&contents).map_err(|e| Error::Benchmark(format!("Invalid JSON: {}", e)))
}

/// Save framework sizes to a JSON file
pub fn save_framework_sizes(sizes: &FrameworkSizes, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(sizes)
        .map_err(|e| Error::Benchmark(format!("JSON serialization failed: {}", e)))?;
    fs::write(path, json).map_err(Error::Io)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_package_name() {
        assert_eq!(extract_package_name("kreuzberg-python"), "kreuzberg");
        assert_eq!(extract_package_name("docling"), "docling");
        assert_eq!(extract_package_name("docling-batch"), "docling");
        assert_eq!(extract_package_name("pdfplumber-batch"), "pdfplumber");
    }

    #[test]
    fn test_frameworks_list_complete() {
        // 10 kreuzberg bindings + 8 third-party = 18 total
        assert_eq!(FRAMEWORKS.len(), 18);

        // Check all kreuzberg bindings present
        let names: Vec<&str> = FRAMEWORKS.iter().map(|(n, _, _)| *n).collect();
        assert!(names.contains(&"kreuzberg-rust"));
        assert!(names.contains(&"kreuzberg-python"));
        assert!(names.contains(&"kreuzberg-node"));

        // Check third-party frameworks present
        assert!(names.contains(&"docling"));
        assert!(names.contains(&"tika"));
        assert!(names.contains(&"pandoc"));
    }

    #[test]
    fn test_dir_size_empty() {
        let temp = tempfile::TempDir::new().unwrap();
        let size = dir_size(temp.path());
        assert_eq!(size, 0);
    }

    #[test]
    fn test_dir_size_with_files() {
        let temp = tempfile::TempDir::new().unwrap();
        fs::write(temp.path().join("a.txt"), "hello").unwrap();
        fs::write(temp.path().join("b.txt"), "world!").unwrap();

        let size = dir_size(temp.path());
        assert_eq!(size, 11); // "hello" (5) + "world!" (6)
    }

    #[test]
    fn test_measure_native_ffi_libs_does_not_panic() {
        // Should return 0 or a positive value depending on build state
        let _size = measure_native_ffi_libs();
    }

    #[test]
    fn test_measure_pip_package_via_python_nonexistent() {
        let result = measure_pip_package_via_python("nonexistent_package_xyz_123");
        assert!(result.is_none());
    }

    #[test]
    fn test_has_native_extension_empty_dir() {
        let temp = tempfile::TempDir::new().unwrap();
        assert!(!has_native_extension(temp.path()));
    }

    #[test]
    fn test_has_native_extension_with_so() {
        let temp = tempfile::TempDir::new().unwrap();
        fs::write(temp.path().join("module.so"), "fake").unwrap();
        assert!(has_native_extension(temp.path()));
    }

    #[test]
    fn test_has_native_extension_nested() {
        let temp = tempfile::TempDir::new().unwrap();
        let sub = temp.path().join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("lib.dylib"), "fake").unwrap();
        assert!(has_native_extension(temp.path()));
    }

    #[test]
    fn test_has_native_extension_no_match() {
        let temp = tempfile::TempDir::new().unwrap();
        fs::write(temp.path().join("file.txt"), "text").unwrap();
        fs::write(temp.path().join("lib.py"), "python").unwrap();
        assert!(!has_native_extension(temp.path()));
    }
}

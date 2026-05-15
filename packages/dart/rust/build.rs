fn main() {
    // Re-run whenever any Rust source changes.
    println!("cargo:rerun-if-changed=src");

    let status = std::process::Command::new("flutter_rust_bridge_codegen")
        .args(["generate", "--config-file", "flutter_rust_bridge.yaml"])
        .status()
        .expect("flutter_rust_bridge_codegen not found on PATH; install via `dart pub global activate flutter_rust_bridge_codegen`");

    if !status.success() {
        panic!("flutter_rust_bridge_codegen generate failed (exit code: {status})");
    }
}

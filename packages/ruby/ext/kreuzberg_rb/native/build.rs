#[cfg(target_os = "macos")]
fn main() {
    if let Ok(cargo_manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let lib_path = std::path::Path::new(&cargo_manifest_dir)
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("target/release"))
            .expect("Failed to construct lib path");
        println!("cargo:rustc-link-search={}", lib_path.display());
    }
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/.");
}

#[cfg(target_os = "linux")]
fn main() {
    if let Ok(cargo_manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let lib_path = std::path::Path::new(&cargo_manifest_dir)
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("target/release"))
            .expect("Failed to construct lib path");
        println!("cargo:rustc-link-search={}", lib_path.display());
    }
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/.");
}

#[cfg(target_os = "windows")]
fn main() {
    if let Ok(cargo_manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let lib_path = std::path::Path::new(&cargo_manifest_dir)
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("target/release"))
            .expect("Failed to construct lib path");
        println!("cargo:rustc-link-search={}", lib_path.display());
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn main() {}

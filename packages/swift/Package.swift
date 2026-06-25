// swift-tools-version: 6.0
import PackageDescription

// NOTE: Run `cargo build -p xberg-swift` and then rerun `alef generate`
// before `swift build`. Alef materializes the swift-bridge Swift/C outputs into
// Sources/RustBridge and Sources/RustBridgeC when the Cargo build output exists.
// See README.md for the full workflow.
let package = Package(
  name: "Xberg",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  products: [
    .library(name: "Xberg", targets: ["Xberg"])
  ],
  targets: [
    // RustBridgeC: pure C/headers target. Swift files in RustBridge import this
    // to access C types (RustStr, etc.) produced by swift-bridge.
    // publicHeadersPath: "." exposes RustBridgeC.h to dependents.
    .target(
      name: "RustBridgeC",
      path: "Sources/RustBridgeC",
      publicHeadersPath: "."
    ),
    // RustBridge: Swift wrapper around the Rust static library.
    // Depends on RustBridgeC so the generated Swift files can use the C types.
    // linkerSettings wire the Rust staticlibs (libxberg_swift.a and libxberg_ffi.a)
    // produced by `cargo build -p xberg-swift` and the FFI crate so
    // `swift build` / `swift test` can resolve the `__swift_bridge__$*` and FFI C symbols.
    // Both target/release and target/debug are searched so either cargo profile works.
    // The FFI library is needed because the generated Swift service API code (App.swift)
    // calls FFI functions directly via @_silgen_name declarations.
    .target(
      name: "RustBridge",
      dependencies: ["RustBridgeC"],
      path: "Sources/RustBridge",
      linkerSettings: [
        .unsafeFlags([
          "-L../../target/release",
          "-L../../target/debug",
        ]),
        .linkedLibrary("xberg_swift"),
        .linkedLibrary("xberg_ffi"),
        .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
      ]
    ),
    .target(
      name: "Xberg", dependencies: ["RustBridge"],
      path: "Sources/Xberg",
      exclude: ["LICENSE"]),
    .testTarget(
      name: "XbergTests", dependencies: ["Xberg"],
      path: "Tests/XbergTests"),
  ]
)

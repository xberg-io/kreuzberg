// swift-tools-version: 5.9
import PackageDescription

// NOTE: Run `cargo build -p kreuzberg-swift` before `swift build`.
// That step generates Sources/RustBridge/{kreuzberg-swift,SwiftBridgeCore}.swift and the headers.
// See BUILDING.md for the full workflow.
let package = Package(
    name: "Kreuzberg",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    products: [
        .library(name: "Kreuzberg", targets: ["Kreuzberg"]),
    ],
    targets: [
        // RustBridgeC: pure C/headers target. Swift files in RustBridge import this
        // to access C types (RustStr, etc.) produced by swift-bridge.
        .target(
            name: "RustBridgeC",
            path: "Sources/RustBridgeC",
            publicHeadersPath: "."
        ),
        // RustBridge: Swift wrapper around the Rust static library.
        // Depends on RustBridgeC so Swift files here can use the C types.
        .target(
            name: "RustBridge",
            dependencies: ["RustBridgeC"],
            path: "Sources/RustBridge",
            linkerSettings: [
                .linkedLibrary("kreuzberg_swift"),
                .unsafeFlags(["-L../../target/debug"]),
            ]
        ),
        .target(name: "Kreuzberg", dependencies: ["RustBridge"], path: "Sources/Kreuzberg"),
        .testTarget(name: "KreuzbergTests", dependencies: ["Kreuzberg"], path: "Tests/KreuzbergTests"),
    ]
)

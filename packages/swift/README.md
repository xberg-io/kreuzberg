# Kreuzberg

High-performance document intelligence library

## Installation

Add to your `Package.swift`:

```swift
.package(path: "packages/swift"),
```

## Building

```sh
cargo build -p kreuzberg-swift
OUT=$(ls -dt target/debug/build/kreuzberg-swift-*/out 2>/dev/null | head -1)
cat "$OUT/SwiftBridgeCore.h" "$OUT/kreuzberg-swift/kreuzberg-swift.h" \
    > packages/swift/Sources/RustBridgeC/RustBridgeC.h
{ echo "import RustBridgeC"; cat "$OUT/SwiftBridgeCore.swift"; } \
    > packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{ echo "import RustBridgeC"; cat "$OUT/kreuzberg-swift/kreuzberg-swift.swift"; } \
    > packages/swift/Sources/RustBridge/kreuzberg-swift.swift
swift build --package-path packages/swift
swift test --package-path packages/swift
```

The generated `Sources/RustBridgeC` and `Sources/RustBridge` artifacts are
rewritten after each Cargo clean or rebuild.

## License

Elastic-2.0

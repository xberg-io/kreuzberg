#!/bin/bash
# Setup Swift bridge files after cargo build

set -e

# Find the most recently built output directory.
# Portable across macOS (BSD stat) and Linux (GNU stat): use `ls -td` on the
# globbed directory list and pick the first entry (newest mtime).
# shellcheck disable=SC2012
OUT=$(ls -1td target/release/build/xberg-swift-*/out 2>/dev/null | head -1)
if [ -z "$OUT" ] || [ ! -d "$OUT" ]; then
  echo "ERROR: Could not find swift-bridge build output in target/release/build/"
  exit 1
fi

echo "Using swift-bridge output from: $OUT"

# Fix swift-bridge visibility: make 'var ptr' and 'var isOwned' properties public for internal type conversion
fixVisibility() {
  sed -e 's/^    var ptr: UnsafeMutableRawPointer$/    public var ptr: UnsafeMutableRawPointer/g' \
    -e 's/^    var isOwned: Bool = true$/    public var isOwned: Bool = true/g'
}

# Ensure target directories exist
mkdir -p packages/swift/Sources/RustBridgeC
mkdir -p packages/swift/Sources/RustBridge

# Copy C headers
cat "$OUT/SwiftBridgeCore.h" "$OUT/xberg-swift/xberg-swift.h" \
  >packages/swift/Sources/RustBridgeC/RustBridgeC.h

# Copy Swift bridge files with import statement prepended
{
  printf 'import RustBridgeC\n'
  cat "$OUT/SwiftBridgeCore.swift" | fixVisibility
} >packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{
  printf 'import RustBridgeC\n'
  cat "$OUT/xberg-swift/xberg-swift.swift" | fixVisibility
} >packages/swift/Sources/RustBridge/xberg-swift.swift

echo "Swift-bridge files setup complete"

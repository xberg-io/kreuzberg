#!/usr/bin/env bash
set -euo pipefail

# Validate that ffi-download directory exists
if [ ! -d "ffi-download" ]; then
  echo "✗ Error: ffi-download directory not found"
  exit 1
fi

mkdir -p target/release
mkdir -p target/x86_64-pc-windows-gnu/release
mkdir -p packages/go/v4/internal/ffi
mkdir -p crates/kreuzberg-ffi

echo "Moving FFI artifacts from ffi-download..."
echo ""

# Move library files
LIBRARY_COUNT=0
while IFS= read -r file; do
  filename="$(basename "$file")"
  if [[ "$file" == *"x86_64-pc-windows-gnu"* ]]; then
    cp "$file" target/x86_64-pc-windows-gnu/release/
    echo "✓ Copied $filename to target/x86_64-pc-windows-gnu/release/"
  else
    cp "$file" target/release/
    echo "✓ Copied $filename to target/release/"
  fi
  ((LIBRARY_COUNT++)) || true
done < <(find ffi-download -type f \( -name "libkreuzberg_ffi.*" -o -name "kreuzberg_ffi.*" \))

if [ "$LIBRARY_COUNT" -eq 0 ]; then
  echo "⚠ Warning: No FFI library files found in ffi-download (may be a cross-platform build artifact)"
fi

# Copy header file to Go package (check both flat and nested paths)
HEADER_FOUND=false
if [ -f "ffi-download/kreuzberg.h" ]; then
  cp ffi-download/kreuzberg.h packages/go/v4/internal/ffi/
  echo "✓ Copied kreuzberg.h to packages/go/v4/internal/ffi/"
  HEADER_FOUND=true
elif [ -f "ffi-download/crates/kreuzberg-ffi/kreuzberg.h" ]; then
  cp ffi-download/crates/kreuzberg-ffi/kreuzberg.h packages/go/v4/internal/ffi/
  echo "✓ Copied kreuzberg.h to packages/go/v4/internal/ffi/"
  HEADER_FOUND=true
fi

if [ "$HEADER_FOUND" = false ]; then
  echo "✗ Error: Header file kreuzberg.h not found in ffi-download"
  echo "   Contents of ffi-download:"
  ls -la ffi-download/ || echo "   (unable to list directory)"
  echo "   Contents of ffi-download/crates (if exists):"
  ls -la ffi-download/crates/ 2>/dev/null || echo "   (directory does not exist)"
  exit 1
fi

# Verify header was copied
if [ ! -f "packages/go/v4/internal/ffi/kreuzberg.h" ]; then
  echo "✗ Error: Failed to copy kreuzberg.h to packages/go/v4/internal/ffi/"
  exit 1
fi

# Copy pkg-config file (check both flat and nested paths)
if [ -f "ffi-download/kreuzberg-ffi.pc" ]; then
  cp ffi-download/kreuzberg-ffi.pc crates/kreuzberg-ffi/
  echo "✓ Copied kreuzberg-ffi.pc to crates/kreuzberg-ffi/"
elif [ -f "ffi-download/crates/kreuzberg-ffi/kreuzberg-ffi.pc" ]; then
  cp ffi-download/crates/kreuzberg-ffi/kreuzberg-ffi.pc crates/kreuzberg-ffi/
  echo "✓ Copied kreuzberg-ffi.pc to crates/kreuzberg-ffi/"
else
  echo "⚠ Warning: pkg-config file kreuzberg-ffi.pc not found in ffi-download"
fi

echo ""
echo "Cleaning up ffi-download directory..."
rm -rf ffi-download
echo "✓ Done"

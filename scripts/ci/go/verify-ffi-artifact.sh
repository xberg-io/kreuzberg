#!/usr/bin/env bash
set -euo pipefail

ARTIFACT="${1}"

if [ ! -f "${ARTIFACT}" ]; then
  echo "✗ Artifact not found: ${ARTIFACT}"
  exit 1
fi

echo "=== Verifying artifact structure ==="
tar -tzf "${ARTIFACT}"

cleanup() {
  rm -rf verify-temp
}
trap cleanup EXIT

mkdir -p verify-temp
tar -xzf "${ARTIFACT}" -C verify-temp

REQUIRED_FILES=(
  "xberg-ffi/include/xberg.h"
  "xberg-ffi/share/pkgconfig/xberg-ffi.pc"
)

echo ""
echo "=== Checking required files ==="
for file in "${REQUIRED_FILES[@]}"; do
  if [ -f "verify-temp/$file" ]; then
    echo "✓ Found: $file"
  else
    echo "✗ Missing: $file"
    exit 1
  fi
done

echo ""
echo "=== Checking static library (required for Go) ==="
STATIC_LIB="verify-temp/xberg-ffi/lib/libxberg_ffi.a"
if [ -f "$STATIC_LIB" ]; then
  echo "✓ Found static library: libxberg_ffi.a ($(du -h "$STATIC_LIB" | cut -f1))"
else
  echo "✗ Missing static library: libxberg_ffi.a"
  exit 1
fi

echo ""
echo "=== Checking platform-specific dynamic libraries (optional) ==="
PLATFORM_LIBS_FOUND=0

if find verify-temp/xberg-ffi/lib -name "*.so" -o -name "*.so.*" 2>/dev/null | grep -q .; then
  LIBXBERG=$(find verify-temp/xberg-ffi/lib -name "libxberg_ffi.so*" 2>/dev/null | head -1)
  if [ -n "$LIBXBERG" ]; then
    echo "✓ Found Linux dynamic library: $(basename "$LIBXBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if find verify-temp/xberg-ffi/lib -name "*.dylib" 2>/dev/null | grep -q .; then
  LIBXBERG=$(find verify-temp/xberg-ffi/lib -name "libxberg_ffi.dylib" 2>/dev/null | head -1)
  if [ -n "$LIBXBERG" ]; then
    echo "✓ Found macOS dynamic library: $(basename "$LIBXBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if find verify-temp/xberg-ffi/lib -name "*.dll" 2>/dev/null | grep -q .; then
  LIBXBERG=$(find verify-temp/xberg-ffi/lib -name "xberg_ffi.dll" 2>/dev/null | head -1)
  if [ -n "$LIBXBERG" ]; then
    echo "✓ Found Windows dynamic library: $(basename "$LIBXBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if [ $PLATFORM_LIBS_FOUND -eq 0 ]; then
  echo "  (No dynamic libraries found - static linking only)"
fi

echo ""
echo "✓ Artifact verification passed"

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
  "kreuzberg-ffi/include/kreuzberg.h"
  "kreuzberg-ffi/share/pkgconfig/kreuzberg-ffi.pc"
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
STATIC_LIB="verify-temp/kreuzberg-ffi/lib/libkreuzberg_ffi.a"
if [ -f "$STATIC_LIB" ]; then
  echo "✓ Found static library: libkreuzberg_ffi.a ($(du -h "$STATIC_LIB" | cut -f1))"
else
  echo "✗ Missing static library: libkreuzberg_ffi.a"
  exit 1
fi

echo ""
echo "=== Checking platform-specific dynamic libraries (optional) ==="
PLATFORM_LIBS_FOUND=0

if find verify-temp/kreuzberg-ffi/lib -name "*.so" -o -name "*.so.*" 2>/dev/null | grep -q .; then
  LIBKREUZBERG=$(find verify-temp/kreuzberg-ffi/lib -name "libkreuzberg_ffi.so*" 2>/dev/null | head -1)
  if [ -n "$LIBKREUZBERG" ]; then
    echo "✓ Found Linux dynamic library: $(basename "$LIBKREUZBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if find verify-temp/kreuzberg-ffi/lib -name "*.dylib" 2>/dev/null | grep -q .; then
  LIBKREUZBERG=$(find verify-temp/kreuzberg-ffi/lib -name "libkreuzberg_ffi.dylib" 2>/dev/null | head -1)
  if [ -n "$LIBKREUZBERG" ]; then
    echo "✓ Found macOS dynamic library: $(basename "$LIBKREUZBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if find verify-temp/kreuzberg-ffi/lib -name "*.dll" 2>/dev/null | grep -q .; then
  LIBKREUZBERG=$(find verify-temp/kreuzberg-ffi/lib -name "kreuzberg_ffi.dll" 2>/dev/null | head -1)
  if [ -n "$LIBKREUZBERG" ]; then
    echo "✓ Found Windows dynamic library: $(basename "$LIBKREUZBERG")"
    PLATFORM_LIBS_FOUND=1
  fi
fi

if [ $PLATFORM_LIBS_FOUND -eq 0 ]; then
  echo "  (No dynamic libraries found - static linking only)"
fi

echo ""
echo "=== Checking required FFI symbols ==="
# Derive required symbols dynamically from the Go binding so this check stays
# in sync automatically as the API surface grows. Any C.kreuzberg_* call in
# binding.go must resolve to a compiled symbol — the root cause of issue #871
# was three embedding symbols compiled out by a stale feature flag, which this
# check would have caught.
BINDING_FILE="packages/go/v4/binding.go"
if [ ! -f "$BINDING_FILE" ]; then
  echo "  (binding file not found at $BINDING_FILE — skipping symbol check)"
elif ! command -v nm >/dev/null 2>&1; then
  echo "  (nm not available — skipping symbol check)"
else
  REQUIRED_SYMBOLS=$(grep -o 'C\.kreuzberg_[a-zA-Z0-9_]*' "$BINDING_FILE" | sed 's/C\.//' | sort -u)
  if [ -z "$REQUIRED_SYMBOLS" ]; then
    echo "  (no kreuzberg_ symbols found in $BINDING_FILE — skipping)"
  else
    NM_OUTPUT=$(nm "$STATIC_LIB" 2>/dev/null)
    SYMBOL_ERRORS=0
    while IFS= read -r sym; do
      if echo "$NM_OUTPUT" | grep -qF "$sym"; then
        echo "✓ $sym"
      else
        echo "✗ Missing: $sym"
        SYMBOL_ERRORS=$((SYMBOL_ERRORS + 1))
      fi
    done <<< "$REQUIRED_SYMBOLS"
    if [ $SYMBOL_ERRORS -gt 0 ]; then
      echo ""
      echo "✗ $SYMBOL_ERRORS symbol(s) referenced in $BINDING_FILE missing from libkreuzberg_ffi.a"
      exit 1
    fi
  fi
fi

echo ""
echo "✓ Artifact verification passed"

#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${REPO_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"

source "$REPO_ROOT/scripts/lib/common.sh"
source "$REPO_ROOT/scripts/lib/library-paths.sh"

validate_repo_root "$REPO_ROOT" || exit 1

set +e
echo "=========================================="
echo "Pre-test CGO Configuration"
echo "=========================================="
echo "Operating System: ${RUNNER_OS:-<not set>}"
echo "Go version: $(go version)"
echo ""
echo "=== CGO Settings ==="
echo "CGO_ENABLED=${CGO_ENABLED:-<not set>}"
echo "CGO_CFLAGS=${CGO_CFLAGS:-<not set>}"
echo "CGO_LDFLAGS=${CGO_LDFLAGS:-<not set>}"
echo ""
echo "=== Runtime Library Paths ==="
if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
  echo "PATH (first 500 chars):"
  echo "$PATH" | head -c 500
  echo "..."
else
  echo "LD_LIBRARY_PATH: ${LD_LIBRARY_PATH:-<not set>}"
  echo "DYLD_LIBRARY_PATH: ${DYLD_LIBRARY_PATH:-<not set>}"
  echo "DYLD_FALLBACK_LIBRARY_PATH: ${DYLD_FALLBACK_LIBRARY_PATH:-<not set>}"
fi
echo ""
echo "=== pkg-config ==="
echo "PKG_CONFIG_PATH: ${PKG_CONFIG_PATH:-<not set>}"
if verify_pkg_config; then
  echo "✓ pkg-config can find kreuzberg-ffi"
else
  echo "⚠ pkg-config cannot find kreuzberg-ffi (build may not be complete yet)"
fi
echo ""
echo "=== FFI Library Files ==="
if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
  if ls target/x86_64-pc-windows-gnu/release/libkreuzberg_ffi.* 2>/dev/null; then
    ls -lh target/x86_64-pc-windows-gnu/release/libkreuzberg_ffi.*
  elif ls target/release/libkreuzberg_ffi.* 2>/dev/null; then
    ls -lh target/release/libkreuzberg_ffi.*
  else
    echo "✗ No FFI library found"
  fi
else
  if ls target/release/libkreuzberg_ffi.* 2>/dev/null; then
    ls -lh target/release/libkreuzberg_ffi.*
  else
    echo "✗ No FFI library found"
  fi
fi
echo "=========================================="

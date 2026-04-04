#!/usr/bin/env bash
set -euo pipefail

ffi_lib_dir="${1:-target/release}"

set +e
echo "=========================================="
echo "Go CGO Environment Configuration"
echo "=========================================="
echo "Repository Root: ${GITHUB_WORKSPACE}"
echo "Platform: ${RUNNER_OS:-}"
echo ""
echo "=== pkg-config ==="
echo "PKG_CONFIG_PATH=${PKG_CONFIG_PATH:-<not set>}"
if [ -f "${GITHUB_WORKSPACE}/crates/kreuzberg-ffi/kreuzberg-ffi.pc" ]; then
  echo "✓ kreuzberg-ffi.pc found"
else
  echo "⚠ kreuzberg-ffi.pc not found (may not be built yet)"
fi
echo ""
echo "=== CGO Compilation Settings ==="
echo "CGO_ENABLED=${CGO_ENABLED:-<not set>}"
echo "CGO_CFLAGS=${CGO_CFLAGS:-<not set>}"
echo "CGO_LDFLAGS=${CGO_LDFLAGS:-<not set>}"
echo ""
echo "=== Runtime Library Paths ==="
if [ "${RUNNER_OS:-}" != "Windows" ]; then
  echo "LD_LIBRARY_PATH=${LD_LIBRARY_PATH:-<not set>}"
  echo "DYLD_LIBRARY_PATH=${DYLD_LIBRARY_PATH:-<not set>}"
  echo "DYLD_FALLBACK_LIBRARY_PATH=${DYLD_FALLBACK_LIBRARY_PATH:-<not set>}"
else
  echo "PATH=${PATH:-<not set>}" | head -c 200
  echo "..."
fi
echo ""
echo "=== FFI Library Files ==="
ffi_path="${GITHUB_WORKSPACE}/${ffi_lib_dir}"
if [ -d "$ffi_path" ]; then
  echo "FFI library directory: $ffi_path"
  ls -lh "$ffi_path"/libkreuzberg_ffi.* 2>/dev/null || echo "No libkreuzberg_ffi files found in default target"
else
  echo "FFI library directory does not exist: $ffi_path"
fi
echo "=========================================="

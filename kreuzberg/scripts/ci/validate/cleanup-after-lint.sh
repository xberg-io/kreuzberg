#!/usr/bin/env bash
set -euo pipefail

echo "=== Cleaning up build artifacts ===" >&2

rm -rf target/debug/incremental 2>/dev/null || true
rm -rf target/release/incremental 2>/dev/null || true

if [ -d "target/debug/build" ]; then
  find target/debug/build -type d -name pdfium -path "*/kreuzberg-ffi-*/out/pdfium" -delete 2>/dev/null || {
    echo "Warning: Failed to clean some debug pdfium directories" >&2
  }
fi
if [ -d "target/release/build" ]; then
  find target/release/build -type d -name pdfium -path "*/kreuzberg-ffi-*/out/pdfium" -delete 2>/dev/null || {
    echo "Warning: Failed to clean some release pdfium directories" >&2
  }
fi

find . -type d -name __pycache__ -delete 2>/dev/null || true
find . -type d -name .pytest_cache -delete 2>/dev/null || true

bash scripts/ci/validate/show-disk-space.sh "Disk space after cleanup"

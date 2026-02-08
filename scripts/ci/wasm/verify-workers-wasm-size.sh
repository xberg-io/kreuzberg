#!/usr/bin/env bash
set -euo pipefail

cd crates/kreuzberg-wasm/pkg

wasm_size=$(wc -c <kreuzberg_wasm_bg.wasm)
echo "WASM size (uncompressed): $(numfmt --to=iec-i --suffix=B "$wasm_size")"

if [ "$wasm_size" -gt 15728640 ]; then
  echo "ERROR: WASM bundle exceeds standardized 15MB limit: $wasm_size bytes"
  exit 1
fi

gzip -c kreuzberg_wasm_bg.wasm >kreuzberg_wasm_bg.wasm.gz
gzip_size=$(wc -c <kreuzberg_wasm_bg.wasm.gz)
echo "WASM size (gzipped): $(numfmt --to=iec-i --suffix=B "$gzip_size")"

if [ "$gzip_size" -gt 6291456 ]; then
  echo "ERROR: Gzipped WASM exceeds standardized 6MB limit: $gzip_size bytes"
  exit 1
fi

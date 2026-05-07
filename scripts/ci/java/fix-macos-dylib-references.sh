#!/usr/bin/env bash
set -euo pipefail

if [ "$(uname -s)" != "Darwin" ]; then
  exit 0
fi

ffi="target/release/libkreuzberg_ffi.dylib"
if [ ! -f "$ffi" ]; then
  exit 0
fi


shopt -s nullglob
for ort in target/release/libonnxruntime*.dylib; do
  install_name_tool -change @rpath/"$(basename "$ort")" @loader_path/"$(basename "$ort")" "$ffi" 2>/dev/null || true
done

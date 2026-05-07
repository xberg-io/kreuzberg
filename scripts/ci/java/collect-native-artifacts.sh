#!/usr/bin/env bash
set -euo pipefail

rid="${1:?rid required}"
out="${2:-java-natives/${rid}}"

mkdir -p "$out"

case "$rid" in
windows-x86_64)
  cp -f target/release/kreuzberg_ffi.dll "$out/"
  ;;
macos-x86_64 | macos-arm64)
  cp -f target/release/libkreuzberg_ffi.dylib "$out/"
  ;;
linux-x86_64)
  cp -f target/release/libkreuzberg_ffi.so "$out/"
  ;;
linux-arm64)
  cp -f target/release/libkreuzberg_ffi.so "$out/"
  ;;
*)
  echo "Unsupported rid: $rid" >&2
  exit 1
  ;;
esac

ls -la "$out"

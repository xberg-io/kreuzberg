#!/usr/bin/env bash

set -euo pipefail

target="${CLI_TARGET:-}"

sudo apt-get update
case "$target" in
aarch64-unknown-linux-gnu)
  sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
  ;;
x86_64-unknown-linux-musl)
  # Download musl cross-compiler toolchain with C++ support (includes musl-native libstdc++)
  curl -fsSL https://musl.cc/x86_64-linux-musl-cross.tgz | sudo tar xz -C /opt/
  echo "/opt/x86_64-linux-musl-cross/bin" >> "$GITHUB_PATH"
  # Set C/C++ compilers for cmake builds (kreuzberg-tesseract)
  echo "CC=x86_64-linux-musl-gcc" >> "$GITHUB_ENV"
  echo "CXX=x86_64-linux-musl-g++" >> "$GITHUB_ENV"
  # Set Rust linker for musl target
  echo "CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc" >> "$GITHUB_ENV"
  ;;
*) ;;
esac

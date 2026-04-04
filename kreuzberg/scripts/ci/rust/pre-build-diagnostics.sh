#!/usr/bin/env bash
set -euo pipefail

echo "=== Build Environment Diagnostics ==="
echo "OS: ${RUNNER_OS:-<not set>}"
echo "Arch: ${RUNNER_ARCH:-<not set>}"
echo "Rust toolchain:"
rustc --version
cargo --version
echo "Available targets (installed):"
rustup target list | grep installed || echo "No targets installed"
echo "Environment variables:"
echo "  CARGO_TERM_COLOR=${CARGO_TERM_COLOR:-not set}"
echo "  CARGO_INCREMENTAL=${CARGO_INCREMENTAL:-not set}"
echo "  RUST_BACKTRACE=${RUST_BACKTRACE:-not set}"
echo "  RUSTFLAGS=${RUSTFLAGS:-not set}"
echo "  RUSTC_WRAPPER=${RUSTC_WRAPPER:-not set}"
echo "  TESSDATA_PREFIX=${TESSDATA_PREFIX:-not set}"
echo "Disk space:"
df -h / || du -h / 2>/dev/null | head -1
echo "=== End diagnostics ==="

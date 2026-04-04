#!/usr/bin/env bash

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

source "${REPO_ROOT}/scripts/lib/common.sh"

validate_repo_root "$REPO_ROOT" || exit 1

BINARY_PATH="${BINARY_PATH:-$REPO_ROOT/target/release/benchmark-harness}"

if [ ! -f "$BINARY_PATH" ]; then
  echo "::error::Binary not found at $BINARY_PATH" >&2
  exit 1
fi

chmod +x "$BINARY_PATH"
echo "✓ Restored executable permissions on: $BINARY_PATH"

# Also restore kreuzberg-extract if present (used by kreuzberg-rust and kreuzberg-rust-paddle adapters)
EXTRACT_BINARY="$REPO_ROOT/target/release/kreuzberg-extract"
if [ -f "$EXTRACT_BINARY" ]; then
  chmod +x "$EXTRACT_BINARY"
  echo "✓ Restored executable permissions on: $EXTRACT_BINARY"
fi

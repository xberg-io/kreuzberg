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

# Also restore xberg-cli if present (used by all xberg adapter pipelines)
CLI_BINARY="$REPO_ROOT/target/release/xberg"
if [ -f "$CLI_BINARY" ]; then
  chmod +x "$CLI_BINARY"
  echo "✓ Restored executable permissions on: $CLI_BINARY"
fi

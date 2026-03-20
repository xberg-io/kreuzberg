#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "=== Installing kreuzberg FFI library ==="
go generate github.com/kreuzberg-dev/kreuzberg/packages/go/v4

echo "=== Running tests ==="
go test -v ./...

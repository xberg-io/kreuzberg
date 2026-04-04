#!/usr/bin/env bash

set -euo pipefail

target="${TARGET:?TARGET not set}"
use_cross="${USE_CROSS:-false}"
use_napi_cross="${USE_NAPI_CROSS:-false}"

echo "=== Building Node Native Module ==="
echo "Target: $target"
echo "Use cross: $use_cross"
echo "Use napi-cross: $use_napi_cross"

args=(--platform --release --target "$target" --output-dir ./artifacts)
if [ "$use_napi_cross" = "true" ]; then
  args+=(--use-napi-cross)
fi
if [ "$use_cross" = "true" ]; then
  args+=(--use-cross)
fi

echo "Running: pnpm --filter @kreuzberg/node exec napi build ${args[*]}"
pnpm --filter @kreuzberg/node exec napi build "${args[@]}"

artifacts_dir="crates/kreuzberg-node/artifacts"
echo ""
echo "=== Build Output ==="
ls -lah "$artifacts_dir" 2>/dev/null || echo "Artifacts directory not found!"
echo "=== Checking for .node files ==="
find "$artifacts_dir" -name "*.node" -print 2>/dev/null || echo "No .node files found!"

# Verify that at least one .node file was created
node_files=$(find "$artifacts_dir" -name "*.node" 2>/dev/null | wc -l)
if [ "$node_files" -eq 0 ]; then
  echo "ERROR: Native module build succeeded but no .node file was generated" >&2
  echo "Expected to find .node files in $artifacts_dir" >&2
  exit 1
fi
echo "✓ Found $node_files .node file(s)"

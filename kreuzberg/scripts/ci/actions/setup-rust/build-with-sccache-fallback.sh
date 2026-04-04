#!/usr/bin/env bash
set -euo pipefail

# Usage: build-with-sccache-fallback.sh <cargo command...>
log_file=$(mktemp)
trap 'rm -f "$log_file"' EXIT

echo "Building with sccache (fallback on errors)..."

# Attempt with sccache
if "$@" 2>&1 | tee "$log_file"; then
  echo "✓ Build succeeded with sccache"
  exit 0
fi

# Check for sccache-related errors
if grep -Eq "sccache.*(error|failed)|cache storage failed|dns error|connection (refused|timed out)" "$log_file"; then
  echo "⚠️  sccache failure detected, retrying without cache..."
  export RUSTC_WRAPPER=""
  export SCCACHE_GHA_ENABLED=false

  if "$@"; then
    echo "✓ Build succeeded without sccache (fallback)"
    exit 0
  fi
fi

echo "✗ Build failed"
exit 1

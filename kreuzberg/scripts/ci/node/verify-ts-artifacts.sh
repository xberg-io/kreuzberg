#!/usr/bin/env bash
set -euo pipefail

cd crates/kreuzberg-node

echo "Checking dist directory:"
ls -la dist/ || echo "ERROR: dist directory not found"
echo ""

echo "Checking tarball contents for dist files:"
tgz="$(find . -maxdepth 1 -name "*.tgz" -type f | head -n1 || true)"
if [ -n "$tgz" ]; then
  echo "Found tarball: $tgz"
  tar tzf "$tgz" | grep -c "^package/dist/" | xargs echo "dist files in tarball:"
else
  echo "ERROR: No tarball found"
  exit 1
fi

#!/usr/bin/env bash
set -euo pipefail

label="${1:-Node setup}"

echo "=== ${label} ==="
echo "Node version: $(node --version)"
echo "pnpm version: $(pnpm --version)"
echo "tsx availability: $(command -v tsx || echo 'NOT FOUND')"
echo "pnpm workspace structure:"
pnpm list --depth=0 || true

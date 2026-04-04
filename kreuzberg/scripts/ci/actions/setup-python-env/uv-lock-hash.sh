#!/usr/bin/env bash
set -euo pipefail

if [ -f uv.lock ]; then
  if command -v sha256sum >/dev/null 2>&1; then
    hash="$(sha256sum uv.lock | cut -d' ' -f1)"
  else
    hash="$(shasum -a 256 uv.lock | cut -d' ' -f1)"
  fi
else
  echo "uv.lock not found, using fallback hash" >&2
  hash="no-uv-lock"
fi

echo "value=$hash" >>"$GITHUB_OUTPUT"

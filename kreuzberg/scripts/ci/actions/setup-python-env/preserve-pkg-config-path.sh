#!/usr/bin/env bash
set -euo pipefail

if [ -n "${PKG_CONFIG_PATH:-}" ]; then
  echo "PKG_CONFIG_PATH=${PKG_CONFIG_PATH}" >>"$GITHUB_ENV"
  echo "Preserved PKG_CONFIG_PATH: $PKG_CONFIG_PATH" >&2
fi

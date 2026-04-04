#!/usr/bin/env bash
set -euo pipefail

lang="${1:-}"
if [ -z "$lang" ]; then
  echo "Usage: $0 <lang>" >&2
  exit 2
fi

fixtures_dir="${FIXTURES_DIR:-fixtures}"
output_dir="${OUTPUT_DIR:-e2e}"

cargo run -p kreuzberg-e2e-generator -- generate --lang "$lang" --fixtures "$fixtures_dir" --output "$output_dir"

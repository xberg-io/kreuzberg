#!/usr/bin/env bash
# Build the documentation site (Zensical / MkDocs, doc dependency group).
#
# Usage:
#   scripts/ci/docs/build.sh
#   scripts/ci/docs/build.sh --strict --log-file /tmp/build-log.txt
#
# Caching: use astral-sh/setup-uv with enable-cache in CI; this script only runs uv.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

strict=false
log_file=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --strict)
      strict=true
      shift
      ;;
    --log-file)
      if [[ $# -lt 2 ]]; then
        echo "error: --log-file requires a path" >&2
        exit 2
      fi
      log_file="$2"
      shift 2
      ;;
    *)
      echo "usage: $0 [--strict] [--log-file PATH]" >&2
      exit 2
      ;;
  esac
done

uv_sync() {
  uv sync --group doc --no-editable --no-install-workspace --no-install-project
}

zensical_build() {
  if [[ "$strict" == true ]]; then
    uv run --no-sync zensical build --clean --strict
  else
    uv run --no-sync zensical build --clean
  fi
}

if [[ -n "$log_file" ]]; then
  set -o pipefail
  mkdir -p "$(dirname "$log_file")"
  : >"$log_file"
  uv_sync 2>&1 | tee -a "$log_file"
  zensical_build 2>&1 | tee -a "$log_file"
else
  uv_sync
  zensical_build
fi

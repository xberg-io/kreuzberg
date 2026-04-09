#!/usr/bin/env bash
# Run textlint prose linting against docs/**/*.md.
#
# Usage:
#   scripts/ci/docs/textlint.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

npx textlint "docs/**/*.md"

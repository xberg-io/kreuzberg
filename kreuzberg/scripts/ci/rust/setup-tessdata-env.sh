#!/usr/bin/env bash
set -euo pipefail

source scripts/lib/tessdata.sh
setup_tessdata
echo "TESSDATA_PREFIX=${TESSDATA_PREFIX}" >>"$GITHUB_ENV"

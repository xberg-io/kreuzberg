#!/usr/bin/env bash
set -euo pipefail

label="${1:?label required}"

rm -rf ".tesseract-cache/${label}"
rm -rf ".xdg-cache/${label}"

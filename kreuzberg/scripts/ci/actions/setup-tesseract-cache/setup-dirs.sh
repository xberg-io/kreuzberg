#!/usr/bin/env bash
set -euo pipefail

label="${1:?label required}"

mkdir -p ".tesseract-cache/${label}"
mkdir -p ".xdg-cache/${label}"

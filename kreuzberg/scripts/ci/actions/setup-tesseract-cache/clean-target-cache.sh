#!/usr/bin/env bash
set -euo pipefail

rust_target="${1:?rust target required}"
rm -rf "target/${rust_target}/kreuzberg-tesseract-cache"

#!/usr/bin/env bash
set -euo pipefail

label="${1:-Disk space}"
echo "=== ${label} ===" >&2
df -h / >&2

echo "Disk info:" >&2
df -B1 / | tail -1 >&2 || true

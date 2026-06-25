#!/usr/bin/env bash
set -euo pipefail

echo "=== Running Docker CLI feature tests ==="
python3 scripts/ci/docker/test_docker.py --image "xberg:cli" --variant cli --verbose

#!/usr/bin/env bash
# CI wrapper for Docker configuration testing
# Tests volume mounts, config formats, and environment variable overrides

set -euo pipefail

variant="${1:?missing variant}"

echo "=== Running Docker configuration tests (${variant}) ==="

# Run the comprehensive config test script
# The script expects the image to already be built and tagged
exec ./scripts/test/test-docker-config-local.sh --image "kreuzberg:${variant}" --variant "${variant}"

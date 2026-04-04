#!/usr/bin/env bash
set -euo pipefail

# Only install dev dependencies in CI
# - benchmark group: only for benchmark runs
# - doc group: only for doc publishing workflow
uv sync --all-packages --group dev --all-extras --no-install-project --no-install-workspace

if ! uv run python -c "import cv2; assert hasattr(cv2, 'cvtColor')" 2>/dev/null; then
  echo "⚠️  Detected broken cv2 module, reinstalling OpenCV packages..." >&2
  uv pip uninstall opencv-contrib-python opencv-python-headless --quiet 2>/dev/null || true
  uv pip install opencv-python-headless opencv-contrib-python
  echo "✅ OpenCV packages reinstalled" >&2
fi

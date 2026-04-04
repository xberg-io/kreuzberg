#!/usr/bin/env bash

set -euo pipefail

echo "=== Python CI Disk Space Cleanup ==="

echo "Initial disk space:"
df -h /

echo "=== Removing unnecessary packages ==="
sudo rm -rf /usr/share/dotnet /usr/local/lib/android /opt/ghc /opt/hostedtoolcache/CodeQL || true
sudo rm -rf /usr/local/share/boost /usr/local/lib/node_modules /opt/microsoft /usr/local/.ghcup || true

echo "=== Removing large apt packages ==="
sudo apt-get remove --yes -o APT::AutoRemove::SuggestsImportant=false \
  '^ghc-' 'php.*' 'powershell' 'azure-cli' 'google-cloud-sdk' 2>/dev/null || true

echo "=== Cleaning apt cache ==="
sudo apt-get autoremove --yes || true
sudo apt-get clean || true
sudo rm -rf /var/lib/apt/lists/* || true

echo "=== Cleaning Docker images and containers ==="
docker system prune -af --volumes || true
docker builder prune -af || true

echo "=== Cleaning journalctl logs ==="
sudo journalctl --vacuum=50M || true

echo "=== Cleaning Python pip cache ==="
sudo rm -rf ~/.cache/pip /tmp/pip-* /tmp/tmp* || true
sudo rm -rf /opt/pipx_bin /opt/pipx || true

echo "=== Cleaning temporary build artifacts ==="
rm -rf ~/.cargo/registry/cache ~/.cargo/git/db || true
rm -rf /tmp/* || true

echo "=== Cleaning HuggingFace cache (before tests) ==="
rm -rf ~/.cache/huggingface/datasets ~/.cache/huggingface/modules || true

echo "=== Final disk space after cleanup ==="
df -h /

echo "Disk info:"
df -B1 / | tail -1 || true

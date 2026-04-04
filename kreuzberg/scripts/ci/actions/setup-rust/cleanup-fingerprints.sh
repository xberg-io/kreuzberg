#!/usr/bin/env bash
set -euo pipefail

# This script cleans up stale cargo fingerprints that can cause build failures
# especially when cached artifacts are restored on a different machine/architecture

echo "=== Cleaning Cargo Fingerprints ==="

# Remove incremental compilation caches (primary source of cross-machine corruption)
echo "Removing incremental compilation caches..."
find target -type d -name "incremental" -exec rm -rf {} + 2>/dev/null || true

# Remove ALL fingerprint directories (not just workspace crates)
# Stale dependency fingerprints also cause "failed to read fingerprint" errors
echo "Cleaning all fingerprint directories..."
find target -type d -name ".fingerprint" -exec rm -rf {} + 2>/dev/null || true

# Remove .cargo-ok markers that can be stale
find target -name ".cargo-ok" -delete 2>/dev/null || true

# Windows-specific cleanup: Remove registry state files that can be corrupted
if [[ "$RUNNER_OS" == "Windows" ]] || [[ "${OS:-}" == "Windows_NT" ]]; then
  echo "Detected Windows platform - performing Windows-specific cleanup..."

  # Clear the cargo registry index cache which can have encoding issues on Windows
  if [ -d ~/.cargo/registry/index ]; then
    rm -rf ~/.cargo/registry/index
    echo "  Removed cargo registry index"
  fi

  # Remove cargo registry cache OK marker
  rm -f ~/.cargo/registry/cache/.cargo-ok 2>/dev/null || true

  # Force cargo to rebuild registry state
  cargo metadata --quiet 2>/dev/null || true
fi

# Verify cargo can still access its state
echo "Verifying Cargo state..."
if ! cargo --version &>/dev/null; then
  echo "ERROR: Cargo is broken after cleanup!"
  exit 1
fi

echo "Fingerprint cleanup completed successfully"

#!/usr/bin/env bash
set -euo pipefail

OUTPUT_FILE="${1:-artifact-staging/xberg-ffi/README.md}"

cat >"${OUTPUT_FILE}" <<'EOF'
# Xberg FFI Installation Guide

## System-wide installation (requires sudo):
```bash
tar -xzf go-ffi-*.tar.gz
cd xberg-ffi
sudo cp -r lib/* /usr/local/lib/
sudo cp -r include/* /usr/local/include/
sudo cp -r share/* /usr/local/share/
sudo ldconfig  # Linux only
```

## User-local installation:
```bash
tar -xzf go-ffi-*.tar.gz
cd xberg-ffi
cp -r {lib,include,share} ~/.local/
export PKG_CONFIG_PATH="$HOME/.local/share/pkgconfig:$PKG_CONFIG_PATH"
export LD_LIBRARY_PATH="$HOME/.local/lib:$LD_LIBRARY_PATH"  # Linux
export DYLD_FALLBACK_LIBRARY_PATH="$HOME/.local/lib:$DYLD_FALLBACK_LIBRARY_PATH"  # macOS
```

## Using with Go:
```bash
pkg-config --modversion xberg-ffi  # Verify installation
go get github.com/xberg-io/xberg/packages/go/v4@latest
```
EOF

echo "✓ Installation README generated: ${OUTPUT_FILE}"

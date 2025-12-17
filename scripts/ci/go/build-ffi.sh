#!/usr/bin/env bash
# Build FFI library for Go bindings
# Used by: ci-go.yaml - Build FFI library step
# Supports: Unix (Linux/macOS)
#
# Environment Variables:
# - ORT_STRATEGY: Should be set to 'system' for using system ONNX Runtime
# - ORT_LIB_LOCATION: Path to ONNX Runtime lib directory
# - ORT_SKIP_DOWNLOAD: Set to 1 to skip downloading ONNX Runtime
# - ORT_PREFER_DYNAMIC_LINK: Set to 1 for dynamic linking

set -euo pipefail

echo "Building for Unix target"

# Configure ONNX Runtime environment for macOS and Linux
if [[ -n "${ORT_LIB_LOCATION:-}" ]]; then
	echo "=== ONNX Runtime Configuration (Unix) ==="
	echo "ORT_STRATEGY: ${ORT_STRATEGY:-}"
	echo "ORT_LIB_LOCATION: ${ORT_LIB_LOCATION}"
	echo "ORT_SKIP_DOWNLOAD: ${ORT_SKIP_DOWNLOAD:-}"
	echo "ORT_PREFER_DYNAMIC_LINK: ${ORT_PREFER_DYNAMIC_LINK:-}"

	# Ensure RUSTFLAGS includes -L flag for library directory
	if [[ -n "${RUSTFLAGS:-}" ]]; then
		if [[ ! "$RUSTFLAGS" =~ "-L" ]]; then
			export RUSTFLAGS="${RUSTFLAGS} -L ${ORT_LIB_LOCATION}"
		fi
	else
		export RUSTFLAGS="-L ${ORT_LIB_LOCATION}"
	fi
	echo "RUSTFLAGS: ${RUSTFLAGS}"
fi

cargo build -p kreuzberg-ffi --release

#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

source "$REPO_ROOT/scripts/lib/tessdata.sh"
setup_tessdata

# Set library paths for native dependencies (ONNX Runtime, Pdfium)
export DYLD_LIBRARY_PATH="/opt/homebrew/lib:${REPO_ROOT}/target/release${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"
export ORT_DYLIB_PATH="${ORT_DYLIB_PATH:-/opt/homebrew/lib/libonnxruntime.dylib}"

cd "${REPO_ROOT}/packages/csharp"
dotnet test Kreuzberg.Tests/Kreuzberg.Tests.csproj -c Release

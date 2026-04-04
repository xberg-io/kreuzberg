#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

source "${REPO_ROOT}/scripts/lib/common.sh"
source "${REPO_ROOT}/scripts/lib/library-paths.sh"

validate_repo_root "$REPO_ROOT" || exit 1

"${REPO_ROOT}/scripts/download_pdfium_runtime.sh"

setup_go_paths "$REPO_ROOT"

cd "${REPO_ROOT}/packages/go/v4"

# Check if we're in CI mode (detect from environment variables or command line flags)
verbose_mode="${VERBOSE_MODE:-${CI:-false}}"
is_ci="${CI:-}"

# Parse command line flags
while [[ $# -gt 0 ]]; do
  case $1 in
  --verbose | -v)
    verbose_mode=true
    shift
    ;;
  *)
    shift
    ;;
  esac
done

# Set up Go test flags
go_test_flags=("-timeout" "10m")

# In CI or verbose mode, enable detailed output
if [ "$verbose_mode" = "true" ] || [ -n "$is_ci" ]; then
  go_test_flags+=("-v")
  echo "Running Go tests with verbose output..."
fi

# Print environment information for debugging
if [ -n "$is_ci" ] || [ "$verbose_mode" = "true" ]; then
  echo "Environment Information:"
  echo "  Go version: $(go version)"
  echo "  Working directory: $(pwd)"
  echo "  LD_LIBRARY_PATH: ${LD_LIBRARY_PATH:-<not set>}"
  echo "  DYLD_LIBRARY_PATH: ${DYLD_LIBRARY_PATH:-<not set>}"
  echo "  CGO_ENABLED: ${CGO_ENABLED:-<not set>}"
  echo "  CGO_CFLAGS: ${CGO_CFLAGS:-<not set>}"
  echo "  CGO_LDFLAGS: ${CGO_LDFLAGS:-<not set>}"
  echo ""
fi

# Export RUST_BACKTRACE for better error output on segfault
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

# Run tests with better error reporting
echo "Starting Go tests..."
go test "${go_test_flags[@]}" ./... || {
  exit_code=$?
  echo ""
  echo "ERROR: Go tests failed with exit code $exit_code"
  echo "This may be a segmentation fault. Check the output above for stack traces."
  exit $exit_code
}

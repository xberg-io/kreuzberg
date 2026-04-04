#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

source "${REPO_ROOT}/scripts/lib/common.sh"
source "${REPO_ROOT}/scripts/lib/library-paths.sh"

validate_repo_root "$REPO_ROOT" || exit 1

"${REPO_ROOT}/scripts/download_pdfium_runtime.sh"

setup_go_paths "$REPO_ROOT"

cd "${REPO_ROOT}/packages/go/v4"

# Usage information
usage() {
  cat <<EOF
Usage: $0 [OPTIONS]

Debug script for running Go tests with detailed output to help identify segfaults.

OPTIONS:
  -t, --test <pattern>     Run only tests matching pattern (e.g., TestExtraction)
  -r, --race              Enable Go race detector
  -c, --coverage          Generate coverage report
  -l, --list              List all available tests
  -h, --help              Show this help message

ENVIRONMENT VARIABLES:
  RUST_BACKTRACE          Set to 'full' for detailed Rust backtraces (default: 1)

EXAMPLES:
  # Run a specific test with debugging
  $0 --test TestExtraction

  # Run all tests with race detection
  $0 --race

  # List all available tests
  $0 --list

  # Run with coverage
  $0 --coverage
EOF
}

# Set defaults
test_pattern=""
use_race=false
coverage=false
list_tests=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
  -t | --test)
    test_pattern="$2"
    shift 2
    ;;
  -r | --race)
    use_race=true
    shift
    ;;
  -c | --coverage)
    coverage=true
    shift
    ;;
  -l | --list)
    list_tests=true
    shift
    ;;
  -h | --help)
    usage
    exit 0
    ;;
  *)
    echo "Unknown option: $1"
    usage
    exit 1
    ;;
  esac
done

# List available tests
if [ "$list_tests" = true ]; then
  echo "Available Go tests:"
  grep -rn "^func Test" . --include="*_test.go" | awk -F: '{print "  " $3}' | sort | uniq
  exit 0
fi

# Build test flags
go_test_flags=("-v" "-timeout" "10m")

if [ "$use_race" = true ]; then
  go_test_flags+=("-race")
  echo "Running tests with race detector enabled..."
fi

if [ "$coverage" = true ]; then
  go_test_flags+=("-coverprofile=coverage.out" "-covermode=atomic")
  echo "Running tests with coverage instrumentation..."
fi

# Set up environment
export RUST_BACKTRACE="${RUST_BACKTRACE:-full}"
export RUST_LIB_BACKTRACE="${RUST_LIB_BACKTRACE:-1}"

echo "Environment Information:"
echo "  Go version: $(go version)"
echo "  Working directory: $(pwd)"
echo "  LD_LIBRARY_PATH: ${LD_LIBRARY_PATH:-<not set>}"
echo "  DYLD_LIBRARY_PATH: ${DYLD_LIBRARY_PATH:-<not set>}"
echo "  CGO_ENABLED: ${CGO_ENABLED:-<not set>}"
echo "  RUST_BACKTRACE: ${RUST_BACKTRACE:-<not set>}"
echo ""

# Run tests
if [ -n "$test_pattern" ]; then
  echo "Running tests matching pattern: $test_pattern"
  go test "${go_test_flags[@]}" -run "$test_pattern" ./... || {
    exit_code=$?
    echo ""
    echo "ERROR: Test '$test_pattern' failed with exit code $exit_code"
    exit $exit_code
  }
else
  echo "Running all Go tests with verbose output..."
  go test "${go_test_flags[@]}" ./... || {
    exit_code=$?
    echo ""
    echo "ERROR: Go tests failed with exit code $exit_code"
    echo "This may be a segmentation fault. Check the output above for stack traces."
    echo ""
    echo "Tips for debugging:"
    echo "  1. Run a specific test: $0 --test <TestName>"
    echo "  2. Enable race detector: $0 --race"
    echo "  3. See all tests: $0 --list"
    exit $exit_code
  }
fi

# Print coverage report if generated
if [ "$coverage" = true ] && [ -f "coverage.out" ]; then
  echo ""
  echo "Coverage report generated: coverage.out"
  echo "View coverage: go tool cover -html=coverage.out"
fi

echo ""
echo "✓ All tests passed!"

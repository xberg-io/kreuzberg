#!/usr/bin/env bash
# Compute deterministic hash for cache key generation
#
# Usage:
#   compute-hash.sh <glob-pattern> [glob-pattern...]
#   compute-hash.sh --files <file1> <file2> ...
#   compute-hash.sh --dirs <dir1> <dir2> ...
#
# Examples:
#   compute-hash.sh "crates/kreuzberg/**/*.rs" "crates/kreuzberg-ffi/**/*.rs"
#   compute-hash.sh --files Cargo.lock uv.lock
#   compute-hash.sh --dirs crates/kreuzberg/ crates/kreuzberg-ffi/

set -euo pipefail

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

error() {
  echo -e "${RED}Error: $*${NC}" >&2
  exit 1
}

info() {
  echo -e "${GREEN}$*${NC}" >&2
}

warn() {
  echo -e "${YELLOW}$*${NC}" >&2
}

# Check if sha256sum or shasum is available
if command -v sha256sum &>/dev/null; then
  HASH_CMD="sha256sum"
elif command -v shasum &>/dev/null; then
  HASH_CMD="shasum -a 256"
else
  error "Neither sha256sum nor shasum found in PATH"
fi

# Mode detection
MODE="glob"
if [[ "${1:-}" == "--files" ]]; then
  MODE="files"
  shift
elif [[ "${1:-}" == "--dirs" ]]; then
  MODE="dirs"
  shift
fi

if [[ $# -eq 0 ]]; then
  error "No input provided. Usage: $0 <pattern...> or $0 --files <file...> or $0 --dirs <dir...>"
fi

# Temporary file for collecting hashes
TEMP_HASHES=$(mktemp)
trap 'rm -f "$TEMP_HASHES"' EXIT

case "$MODE" in
files)
  # Hash specific files directly
  for file in "$@"; do
    if [[ -f "$file" ]]; then
      $HASH_CMD "$file" >>"$TEMP_HASHES" 2>/dev/null || warn "Failed to hash: $file"
    else
      warn "File not found: $file"
    fi
  done
  ;;

dirs)
  # Hash all files in directories recursively
  for dir in "$@"; do
    if [[ -d "$dir" ]]; then
      # Find all files (excluding hidden files and directories)
      find "$dir" -type f \
        ! -path "*/.*" \
        ! -path "*/target/*" \
        ! -path "*/node_modules/*" \
        ! -path "*/.venv/*" \
        ! -path "*/dist/*" \
        ! -path "*/build/*" \
        -exec "$HASH_CMD" {} \; >>"$TEMP_HASHES" 2>/dev/null || true
    else
      warn "Directory not found: $dir"
    fi
  done
  ;;

glob)
  # Hash files matching glob patterns
  for pattern in "$@"; do
    # Use find with -path for glob matching
    # Convert glob to find path expression

    if [[ "$pattern" == *"**"* ]]; then
      # Handle ** recursive glob (e.g., "crates/kreuzberg/**/*.rs")
      # Extract the base directory and file extension/name pattern
      base_dir=$(echo "$pattern" | cut -d'*' -f1 | sed 's|/$||')

      # Get the suffix after the ** (e.g., "/*.rs" from "crates/kreuzberg/**/*.rs")
      # Remove everything up to and including **/
      suffix="${pattern#*\*\*/}"

      # Extract filename pattern (e.g., "*.rs" from "/*.rs")
      # Remove leading / if present
      if [[ "$suffix" == /* ]]; then
        name_pattern="${suffix#/}"
      else
        name_pattern="$suffix"
      fi

      if [[ -d "$base_dir" ]]; then
        # Find all files recursively using -name for filename matching
        # This is more portable and reliable than bash regex
        find "$base_dir" -type f \
          ! -path "*/.*" \
          ! -path "*/target/*" \
          ! -path "*/node_modules/*" \
          ! -path "*/.venv/*" \
          -name "$name_pattern" \
          -exec "$HASH_CMD" {} \; 2>/dev/null >>"$TEMP_HASHES" || true
      else
        warn "Directory not found: $base_dir"
      fi
    else
      # Simple glob (no **)
      for file in $pattern; do
        if [[ -f "$file" ]]; then
          $HASH_CMD "$file" >>"$TEMP_HASHES" 2>/dev/null || warn "Failed to hash: $file"
        fi
      done
    fi
  done
  ;;
esac

# Check if we found any files to hash
if [[ ! -s "$TEMP_HASHES" ]]; then
  error "No files found matching the provided patterns"
fi

# Sort hashes (for determinism across different find orders)
# Then hash the combined hashes to get final hash
FINAL_HASH=$(sort "$TEMP_HASHES" | $HASH_CMD | cut -d' ' -f1)

# Truncate to 12 characters for cache key (still 48 bits of entropy)
SHORT_HASH="${FINAL_HASH:0:12}"

# Output the hash
echo "$SHORT_HASH"

# Debug info (to stderr)
FILE_COUNT=$(wc -l <"$TEMP_HASHES")
info "Hashed $FILE_COUNT files → $SHORT_HASH" >&2

#!/usr/bin/env bash

set -euo pipefail

artifacts_dir="${1:?Artifacts directory argument required}"

if [ ! -d "$artifacts_dir" ]; then
  echo "Error: Artifacts directory not found: $artifacts_dir" >&2
  exit 1
fi

echo "Extracting bottle hashes from: $artifacts_dir"

# Function to validate SHA256 format (64 hex characters)
validate_sha256() {
  local sha256="$1"
  if [[ ! $sha256 =~ ^[a-f0-9]{64}$ ]]; then
    echo "Invalid SHA256 format: $sha256" >&2
    return 1
  fi
  return 0
}

# Function to compute and validate SHA256 with verification
compute_sha256() {
  local file="$1"
  local sha256
  sha256=$(shasum -a 256 "$file" | cut -d' ' -f1)

  if ! validate_sha256 "$sha256"; then
    echo "Error: Failed to compute valid SHA256 for $file" >&2
    return 1
  fi

  echo "$sha256"
}

bottle_count=0
for bottle in "$artifacts_dir"/kreuzberg-*.bottle.tar.gz; do
  if [ -f "$bottle" ]; then
    filename="$(basename "$bottle")"

    without_suffix="${filename%.bottle.tar.gz}"
    bottle_tag="${without_suffix##*.}"

    echo "Processing bottle: $filename"

    # Verify file integrity before hashing
    if ! tar -tzf "$bottle" >/dev/null 2>&1; then
      echo "Error: Bottle file is corrupted or not a valid tar.gz: $bottle" >&2
      exit 1
    fi

    if ! sha256=$(compute_sha256 "$bottle"); then
      echo "Error: Failed to compute valid SHA256 for $bottle" >&2
      exit 1
    fi

    echo "${bottle_tag}=${sha256}" >>"${GITHUB_OUTPUT:?GITHUB_OUTPUT not set}"

    echo "  $bottle_tag: $sha256"
    ((bottle_count++)) || true
  fi
done

if [ "$bottle_count" -eq 0 ]; then
  echo "Error: No bottle artifacts found in $artifacts_dir" >&2
  exit 1
fi

echo "Successfully extracted and validated $bottle_count bottle hashes"

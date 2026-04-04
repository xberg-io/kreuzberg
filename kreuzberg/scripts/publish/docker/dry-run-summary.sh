#!/usr/bin/env bash
set -euo pipefail

image="${IMAGE:-}"
version="${VERSION:-}"
tag_suffix="${TAG_SUFFIX:-}"

if [ -z "$image" ] || [ -z "$version" ]; then
  echo "Usage: set IMAGE and VERSION (optional TAG_SUFFIX) env vars" >&2
  exit 2
fi

echo "Dry run requested; Docker image ${image}:${version}${tag_suffix} tested but not pushed." >>"$GITHUB_STEP_SUMMARY"

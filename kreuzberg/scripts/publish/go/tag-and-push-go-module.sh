#!/usr/bin/env bash
set -euo pipefail

tag="${1:?Release tag argument required (e.g. v4.0.0-rc.7)}"

version="${tag#v}"
module_tag="packages/go/v${version}"

if git rev-parse "$module_tag" >/dev/null 2>&1; then
  echo "::notice::Go module tag $module_tag already exists locally; skipping."
  exit 0
fi

# Check if tag exists on remote
if git ls-remote --tags origin | grep -q "refs/tags/${module_tag}$"; then
  echo "::notice::Go module tag $module_tag already exists on remote; skipping."
  exit 0
fi

git tag "$module_tag" "$tag"
git push origin "$module_tag"

echo "✅ Go module tag created: $module_tag"

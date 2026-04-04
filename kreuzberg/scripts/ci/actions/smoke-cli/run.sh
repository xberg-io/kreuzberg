#!/usr/bin/env bash
set -euo pipefail

cli_artifacts_dir="${1:?cli artifacts dir required}"
if [[ "$cli_artifacts_dir" != /* ]]; then
  cli_artifacts_dir="${GITHUB_WORKSPACE}/${cli_artifacts_dir}"
fi

echo "Looking for CLI artifacts in: $cli_artifacts_dir"
ls -la "$cli_artifacts_dir" || true

if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
  cli_archive="$(find "$cli_artifacts_dir" -name "*.zip" -type f | head -n 1)"
  [ -n "$cli_archive" ] || {
    echo "CLI binary not found for Windows" >&2
    exit 1
  }
  tmp="$(mktemp -d)"
  unzip -q "$cli_archive" -d "$tmp"
  cli_bin="$(find "$tmp" -name "kreuzberg.exe" | head -n 1)"
else
  cli_archive="$(find "$cli_artifacts_dir" -name "*.tar.gz" -type f | head -n 1)"
  [ -n "$cli_archive" ] || {
    echo "CLI binary not found" >&2
    exit 1
  }
  tmp="$(mktemp -d)"
  tar -xzf "$cli_archive" -C "$tmp"
  cli_bin="$(find "$tmp" -name "kreuzberg" -type f | head -n 1)"
fi

if [ ! -f "${cli_bin:-}" ]; then
  echo "ERROR: CLI binary extraction failed" >&2
  exit 1
fi

chmod +x "$cli_bin" || true
output="$("$cli_bin" --version)"
echo "$output" | rg -q "kreuzberg" || {
  echo "ERROR: CLI did not produce expected version output" >&2
  echo "Output: $output" >&2
  exit 1
}
echo "✓ CLI binary smoke test passed"

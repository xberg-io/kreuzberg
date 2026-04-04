#!/usr/bin/env bash
set -euo pipefail

wheel_path="${1:-}"

tmp="$(mktemp -d)"
cp -R e2e/smoke/python/. "$tmp"/
pushd "$tmp" >/dev/null

python -m venv .venv
if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
  venv_py=".venv/Scripts/python.exe"
else
  venv_py=".venv/bin/python"
fi

if [[ -n "${KREUZBERG_PDFIUM_PREBUILT:-}" ]]; then
  pdfium_runtime="$tmp/pdfium-runtime"
  mkdir -p "$pdfium_runtime"

  case "${RUNNER_OS:-Linux}" in
  Windows)
    src="$KREUZBERG_PDFIUM_PREBUILT/bin/pdfium.dll"
    filename="pdfium.dll"
    ;;
  macOS)
    src="$KREUZBERG_PDFIUM_PREBUILT/lib/libpdfium.dylib"
    filename="libpdfium.dylib"
    ;;
  Linux)
    src="$KREUZBERG_PDFIUM_PREBUILT/lib/libpdfium.so"
    filename="libpdfium.so"
    ;;
  *)
    echo "Unsupported RUNNER_OS '${RUNNER_OS:-}' for Pdfium staging" >&2
    exit 1
    ;;
  esac

  [ -f "$src" ] || {
    echo "Pdfium runtime not found at $src" >&2
    exit 1
  }
  dest="$pdfium_runtime/$filename"
  cp -f "$src" "$dest"

  case "${RUNNER_OS:-Linux}" in
  Windows)
    scripts_dir="$(dirname "$venv_py")"
    cp -f "$dest" "$scripts_dir/$filename"
    ;;
  macOS) export DYLD_LIBRARY_PATH="$pdfium_runtime:${DYLD_LIBRARY_PATH:-}" ;;
  Linux) export LD_LIBRARY_PATH="$pdfium_runtime:${LD_LIBRARY_PATH:-}" ;;
  esac
fi

"$venv_py" -m pip install --upgrade pip

if [[ -n "$wheel_path" ]]; then
  if [[ "$wheel_path" != /* ]]; then
    wheel_path="${GITHUB_WORKSPACE}/${wheel_path}"
  fi
  echo "Looking for wheels in: $wheel_path"
  if [[ -d "$wheel_path" ]]; then
    wheel_file="$(find "$wheel_path" -name "*.whl" -type f | head -n 1)"
    [ -n "$wheel_file" ] || {
      echo "No wheel found in $wheel_path" >&2
      ls -la "$wheel_path" >&2 || true
      exit 1
    }
    "$venv_py" -m pip install "$wheel_file"
  elif [[ -f "$wheel_path" ]]; then
    "$venv_py" -m pip install "$wheel_path"
  else
    echo "Wheel path does not exist: $wheel_path" >&2
    exit 1
  fi
else
  "$venv_py" -m pip install "${GITHUB_WORKSPACE}/packages/python"
fi

echo "Running comprehensive smoke tests..."
"$venv_py" -m pip install pytest
cp -R "${GITHUB_WORKSPACE}/fixtures" "$tmp"/
cp -R "${GITHUB_WORKSPACE}/e2e/python/tests" "$tmp"/
"$venv_py" -m pytest tests/test_smoke.py -v

popd >/dev/null
echo "✓ Python package smoke test passed (7 comprehensive tests)"

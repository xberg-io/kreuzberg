#!/usr/bin/env bash
set -euo pipefail

mode="${1:-check}"
root="$(git rev-parse --show-toplevel)"

elixir_dirs=(
  packages/elixir
  e2e/elixir
)

failed=0

for dir in "${elixir_dirs[@]}"; do
  full="$root/$dir"
  if [ ! -f "$full/mix.exs" ]; then
    continue
  fi

  echo "==> Linting $dir"
  cd "$full"
  mix deps.get --quiet 2>/dev/null || true

  case "$mode" in
  fix)
    mix format || failed=1
    mix credo suggest --all --strict 2>/dev/null || failed=1
    ;;
  check)
    mix format --check-formatted || failed=1
    mix credo --strict 2>/dev/null || failed=1
    ;;
  esac
done

exit $failed

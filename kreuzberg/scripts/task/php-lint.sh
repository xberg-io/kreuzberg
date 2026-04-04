#!/usr/bin/env bash
set -euo pipefail

mode="${1:-check}"
root="$(git rev-parse --show-toplevel)"

# Ensure packages/php tools are installed
cd "$root/packages/php"
composer install --quiet 2>/dev/null || true

fixer="$root/packages/php/vendor/bin/php-cs-fixer"
phpstan="$root/packages/php/vendor/bin/phpstan"

failed=0

# --- packages/php (has its own config, phpstan.neon, etc.) ---
echo "==> Linting packages/php"
cd "$root/packages/php"
case "$mode" in
fix)
  "$fixer" fix || failed=1
  "$phpstan" analyse --memory-limit=512M || failed=1
  ;;
check)
  "$fixer" fix --dry-run --diff || failed=1
  "$phpstan" analyse --memory-limit=512M || failed=1
  ;;
esac

# --- e2e/php (format only, no phpstan config) ---
if [ -d "$root/e2e/php" ]; then
  echo "==> Linting e2e/php"
  cd "$root/e2e/php"
  case "$mode" in
  fix)
    "$fixer" fix --rules='@PSR12,@PHP82Migration' --path-mode=override . || failed=1
    ;;
  check)
    "$fixer" fix --dry-run --diff --rules='@PSR12,@PHP82Migration' --path-mode=override . || failed=1
    ;;
  esac
fi

# --- examples/php (format only, no phpstan config) ---
if [ -d "$root/examples/php" ]; then
  echo "==> Linting examples/php"
  cd "$root/examples/php"
  case "$mode" in
  fix)
    "$fixer" fix --rules='@PSR12,@PHP82Migration' --path-mode=override . || failed=1
    ;;
  check)
    "$fixer" fix --dry-run --diff --rules='@PSR12,@PHP82Migration' --path-mode=override . || failed=1
    ;;
  esac
fi

exit $failed

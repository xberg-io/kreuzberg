#!/usr/bin/env bash
set -euo pipefail

mode="${1:-check}"
root="$(git rev-parse --show-toplevel)"

# Source SDKMAN if available
if [ -f ~/.sdkman/bin/sdkman-init.sh ]; then
  set +u
  # shellcheck source=/dev/null
  source ~/.sdkman/bin/sdkman-init.sh
  sdk use java 25.0.2-tem 2>/dev/null || true
  sdk use maven 3.9.11 2>/dev/null || true
  set -u
fi

java_dirs=(
  packages/java
  e2e/java
)

failed=0

has_plugin() {
  local pom="$1"
  local plugin="$2"
  grep -q "$plugin" "$pom" 2>/dev/null
}

for dir in "${java_dirs[@]}"; do
  full="$root/$dir"
  if [ ! -f "$full/pom.xml" ]; then
    continue
  fi

  echo "==> Linting $dir"
  cd "$full"

  case "$mode" in
  fix)
    if has_plugin "$full/pom.xml" "spotless-maven-plugin"; then
      mvn -q spotless:apply || failed=1
    fi
    ;;
  check)
    if has_plugin "$full/pom.xml" "spotless-maven-plugin"; then
      mvn -q spotless:check || failed=1
    fi
    if has_plugin "$full/pom.xml" "maven-checkstyle-plugin"; then
      mvn -q checkstyle:check || failed=1
    fi
    if has_plugin "$full/pom.xml" "maven-pmd-plugin"; then
      mvn -q pmd:check pmd:cpd-check || failed=1
    fi
    ;;
  *)
    echo "Usage: $0 [fix|check]" >&2
    exit 2
    ;;
  esac
done

exit $failed

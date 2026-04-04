#!/usr/bin/env bash
set -euo pipefail

label="${1:?label required}"
enable_cache="${2:?enable-cache required (true/false)}"

if [ "$enable_cache" = "true" ]; then
  cache_dir="${GITHUB_WORKSPACE}/.tesseract-cache/${label}"

  echo "TESSERACT_RS_CACHE_DIR=${cache_dir}" >>"$GITHUB_ENV"
  echo "XDG_CACHE_HOME=${GITHUB_WORKSPACE}/.xdg-cache/${label}" >>"$GITHUB_ENV"

  echo "cache-dir=${cache_dir}" >>"$GITHUB_OUTPUT"
  echo "cache-enabled=true" >>"$GITHUB_OUTPUT"

  docker_opts="--env TESSERACT_RS_CACHE_DIR=/io/.tesseract-cache/${label}"
  docker_opts="${docker_opts} --env XDG_CACHE_HOME=/io/.xdg-cache/${label}"
  multiarch=""
  if command -v dpkg-architecture >/dev/null 2>&1; then
    multiarch="$(dpkg-architecture -qDEB_HOST_MULTIARCH 2>/dev/null || true)"
  fi
  if [ -z "$multiarch" ]; then
    case "$(uname -m)" in
    x86_64) multiarch="x86_64-linux-gnu" ;;
    aarch64 | arm64) multiarch="aarch64-linux-gnu" ;;
    esac
  fi
  openssl_lib_dir="/usr/lib"
  if [ -n "$multiarch" ]; then
    openssl_lib_dir="/usr/lib/${multiarch}"
  fi
  docker_opts="${docker_opts} --env OPENSSL_LIB_DIR=${openssl_lib_dir}"
  docker_opts="${docker_opts} --env OPENSSL_INCLUDE_DIR=/usr/include"
  echo "docker-options=${docker_opts}" >>"$GITHUB_OUTPUT"
else
  {
    echo "TESSERACT_RS_CACHE_DIR="
  } >>"$GITHUB_ENV"
  {
    echo "cache-dir="
    echo "cache-enabled=false"
    echo "docker-options="
  } >>"$GITHUB_OUTPUT"
fi

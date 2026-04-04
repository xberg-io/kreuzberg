#!/usr/bin/env bash
set -euo pipefail

python_version="${1:?python version required}"
uv python pin "$python_version"

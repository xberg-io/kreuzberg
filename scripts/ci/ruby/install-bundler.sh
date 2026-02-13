#!/usr/bin/env bash
set -euo pipefail

gem install bundler -v 4.0.3 --no-document || gem install bundler --no-document
bundler --version

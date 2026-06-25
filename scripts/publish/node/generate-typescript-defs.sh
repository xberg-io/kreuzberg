#!/usr/bin/env bash

set -euo pipefail

pnpm --filter "{./crates/xberg-node}" exec napi build --platform --dts index.d.ts
mkdir -p typescript-defs
cp crates/xberg-node/index.d.ts typescript-defs/
cp crates/xberg-node/index.js typescript-defs/ || true

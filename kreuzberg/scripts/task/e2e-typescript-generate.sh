#!/usr/bin/env bash
set -euo pipefail

bash scripts/task/e2e-generate.sh typescript
pnpm biome format --write e2e/typescript/tests
pnpm biome check e2e/typescript/tests

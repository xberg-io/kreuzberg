# Cloudflare Workers Test App

Tests `@kreuzberg/wasm` running inside a Cloudflare Workers environment using `@cloudflare/vitest-pool-workers`.

Related issue: <https://github.com/kreuzberg-dev/kreuzberg/issues/335>

## Setup

```bash
npm install
```

## Run Tests

```bash
npm test
```

## What This Tests

- WASM module initialization (`initWasm()`) in Workers runtime
- Text/HTML/JSON/XML extraction via `extractBytes()`
- Sync extraction via `extractBytesSync()`
- Error handling for corrupted/empty/unknown inputs

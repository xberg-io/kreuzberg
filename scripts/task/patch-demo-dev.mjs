#!/usr/bin/env node
// Generates docs/demo-dev.html from docs/demo.html with CDN URLs replaced
// by the local asset server so no manual editing of demo.html is ever needed.
//
// CDN pattern replaced:
//   https://cdn.jsdelivr.net/npm/@kreuzberg/wasm@*/...
//   → http://localhost:9000/...
//
// The output file is gitignored and regenerated on every `task demo:dev`.

import { readFileSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const src  = join(root, "docs", "demo.html");
const dest = join(root, "docs", "demo-dev.html");
const ASSET_PORT = process.env.ASSET_PORT ?? "9000";

const cdnRe = /https:\/\/cdn\.jsdelivr\.net\/npm\/@kreuzberg\/wasm@[^/'"]+/g;

const patched = readFileSync(src, "utf8")
  .replace(cdnRe, `http://localhost:${ASSET_PORT}`)
  .replace(
    /<title>(.*?)<\/title>/,
    "<title>$1 [local dev]</title>",
  )
  .replace(
    "</body>",
    `  <div style="position:fixed;bottom:12px;right:12px;background:#1a172a;border:1px solid #58FBDA55;color:#58FBDA;font-family:monospace;font-size:11px;padding:6px 10px;border-radius:6px;z-index:9999">
    local dev · assets: localhost:${ASSET_PORT}
  </div>\n</body>`,
  );

writeFileSync(dest, patched, "utf8");
console.log(`patch-demo-dev: docs/demo-dev.html → http://localhost:8001/demo-dev.html`);
console.log(`  assets served from http://localhost:${ASSET_PORT}`);

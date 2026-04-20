---
priority: high
---

# Crate Structure

Version source of truth: root `Cargo.toml` `[workspace.package] version`.

## Workspace crates (`crates/`)

- `kreuzberg` — core library: extraction engine, MIME detection, plugin system, OCR, chunking, embeddings, API/MCP server
- `kreuzberg-cli` — CLI binary; thin wrapper over core with `cli` feature set
- `kreuzberg-ffi` — C FFI layer (`#[no_mangle] extern "C"`); opaque handles, cbindgen headers; used by Go, Java, C# bindings
- `kreuzberg-node` — NAPI-RS Node.js/TypeScript bindings
- `kreuzberg-py` — PyO3 Python bindings
- `kreuzberg-php` — ext-php-rs PHP bindings
- `kreuzberg-wasm` — wasm-bindgen WASM bindings; uses `wasm-target` feature set
- `kreuzberg-paddle-ocr` — PaddleOCR via ONNX Runtime; not available on WASM or Windows
- `kreuzberg-pdfium-render` — forked `pdfium-render` with Kreuzberg patches
- `kreuzberg-tesseract` — Rust bindings for Tesseract OCR

## Out-of-workspace bindings (`packages/`)

- `packages/python/` — PyPI (maturin + PyO3)
- `packages/typescript/` — npm type declarations
- `packages/ruby/` — RubyGems (Magnus); native ext compiled by `rake`
- `packages/php/` — Composer (ext-php-rs)
- `packages/go/v4/` — Go module; cgo over kreuzberg-ffi
- `packages/java/` — Maven; Foreign Function & Memory API over kreuzberg-ffi
- `packages/csharp/` — NuGet; P/Invoke over kreuzberg-ffi
- `packages/elixir/` — Hex; Rustler NIF (workspace member at `packages/elixir/native/kreuzberg_rustler`)
- `packages/r/` — CRAN; extendr (excluded from workspace)

## Tools (`tools/`)

- `tools/e2e-generator` — reads JSON fixtures, generates runnable test suites per language into `e2e/`
- `tools/benchmark-harness` — criterion-based benchmark runner
- `tools/snippet-runner` — executes code snippets from `docs/snippets/` to verify they compile

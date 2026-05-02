---
priority: high
---

# Feature Flag Policy

All features in `crates/kreuzberg/Cargo.toml`.

## WASM-Incompatible Features

Only ORT-dependent paths are WASM-incompatible:

- `paddle-ocr` — ONNX Runtime + native C++ deps: not WASM-safe; no WASM equivalent
- `layout-detection` — depends on ONNX Runtime layout models: not WASM-safe
- `embeddings` — depends on ONNX Runtime sentence-transformer models: not WASM-safe
- `auto-rotate` — depends on ONNX Runtime orientation classifier: not WASM-safe

WASM-safe variants:

- `ocr` (native) → `ocr-wasm` (uses `tesseract-wasm` + safe image deps)
- `excel` (native) → `excel-wasm` (drops `tokio-runtime`)
- `tree-sitter` (native dlopen) → `tree-sitter-wasm` (statically-linked grammar pack)
- `liter-llm` — works on WASM via the upstream `wasm-http` feature; included in `wasm-target`
- `stopwords` — pure-Rust, included in `wasm-target`
- `keywords` — pure-Rust YAKE/RAKE, included in `wasm-target`

The `wasm-target` aggregate composes the complete safe WASM-compatible set:
`pdf, html, xml, email, language-detection, chunking, quality, keywords, office, mdx,
excel-wasm, archives, tree-sitter-wasm, ocr-wasm, liter-llm, stopwords`.

## Experimental (NOT in `full`)

- `pdf-oxide` — pure-Rust PDF text extraction; opt-in only, excluded from both `full` and `formats`

## ORT Variants (Mutually Exclusive)

- `ort-bundled` — downloads official Microsoft ORT binaries; default when OCR/ML features active
- `ort-dynamic` — load ORT from system; only when system ORT is guaranteed present

## Platform-Conditional

- `kreuzberg-paddle-ocr`, `hf-hub`, `pprof` — excluded on `wasm32`
- `ureq`: `rustls` on non-Windows; `native-tls` on Windows

## Aggregate Sets

| Feature       | Description                                                                                        |
| ------------- | -------------------------------------------------------------------------------------------------- |
| `formats`     | All document formats + api/mcp/otel/chunking; no OCR, no ML                                        |
| `full`        | `formats` + ocr + paddle-ocr + layout + embeddings + tree-sitter + liter-llm; excludes `pdf-oxide` |
| `wasm-target` | Full safe WASM-compatible set                                                                      |

## Build Profiles

- `release` — LTO thin, codegen-units=1, strip
- `profiling` — inherits release, retains debug info
- `kreuzberg-wasm` override: `opt-level="z"` (size-optimized)
- `sevenz-rust2`, `zip` override: `opt-level=2` (prevents SIGBUS on macOS ARM64)

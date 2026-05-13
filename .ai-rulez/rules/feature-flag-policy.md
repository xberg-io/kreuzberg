---
priority: high
---

# Feature Flag Policy

All features in `crates/kreuzberg/Cargo.toml`.

## ORT-Incompatible Targets (WASM, Android x86_64 emulator)

Only ORT-dependent paths are incompatible. The same paths block both WASM (no native ORT linkage at all) and the `x86_64-linux-android` emulator triple (no pyke prebuilt; `aarch64-linux-android` does ship a prebuilt and gets full ORT):

- `paddle-ocr` — ONNX Runtime + native C++ deps: not WASM-safe; no Android x86_64 prebuilt
- `layout-detection` — depends on ONNX Runtime layout models: not WASM-safe; no Android x86_64 prebuilt
- `embeddings` — depends on ONNX Runtime sentence-transformer models: not WASM-safe; no Android x86_64 prebuilt
- `auto-rotate` — depends on ONNX Runtime orientation classifier: not WASM-safe; no Android x86_64 prebuilt

Pure-Rust **type-only** companion features expose the public config/result types for the above without pulling in ORT:

- `layout-types` — `LayoutDetectionConfig`, `TableModel`, `BBox`, `DetectionResult`, `LayoutClass`, `LayoutDetection`, `RecognizedTable`. `layout-detection` implies `layout-types`.
- `auto-rotate-types` — `OrientationResult`. `auto-rotate` implies `auto-rotate-types`.
- `embedding-presets` — `EmbeddingPreset` (already existed; pure-Rust preset metadata).

WASM/Android-safe variants:

- `ocr` (native) → `ocr-wasm` (uses `tesseract-wasm` + safe image deps) — Android keeps native `ocr`
- `excel` (native) → `excel-wasm` (drops `tokio-runtime`) — Android keeps native `excel`
- `tree-sitter` (native dlopen) → `tree-sitter-wasm` (statically-linked grammar pack) — Android keeps native `tree-sitter`
- `liter-llm` — works on WASM via the upstream `wasm-http` feature; included in `no-ort-target`
- `stopwords` — pure-Rust, included in `no-ort-target`
- `keywords` — pure-Rust YAKE/RAKE, included in `no-ort-target`

The `no-ort-target` aggregate is the shared no-ORT base used by both `wasm-target` and `android-target`. `wasm-target = no-ort-target + excel-wasm + tree-sitter-wasm + ocr-wasm`. `android-target = no-ort-target + excel + tree-sitter + ocr + api + mcp`.

## Experimental (NOT in `full`)

- `pdf-oxide` — pure-Rust PDF text extraction; opt-in only, excluded from both `full` and `formats`

## ORT Variants (Mutually Exclusive)

- `ort-bundled` — downloads official Microsoft ORT binaries; default when OCR/ML features active
- `ort-dynamic` — load ORT from system; only when system ORT is guaranteed present

## Platform-Conditional

- `kreuzberg-paddle-ocr`, `hf-hub`, `pprof` — excluded on `wasm32`
- `ureq`: `rustls` on non-Windows; `native-tls` on Windows
- `kreuzberg-ffi` and `kreuzberg-dart` cargo dependencies are target-conditional: `cfg(all(target_os = "android", target_arch = "x86_64"))` selects `android-target`; all other targets (including arm64 Android phones) get the full ORT-enabled feature set.

## Aggregate Sets

| Feature          | Description                                                                                        |
| ---------------- | -------------------------------------------------------------------------------------------------- |
| `formats`        | All document formats + api/mcp/otel/chunking; no OCR, no ML                                        |
| `full`           | `formats` + ocr + paddle-ocr + layout + embeddings + tree-sitter + liter-llm; excludes `pdf-oxide` |
| `no-ort-target`  | Pure-Rust base: every capability that does not depend on ONNX Runtime                              |
| `wasm-target`    | `no-ort-target` + excel-wasm + tree-sitter-wasm + ocr-wasm                                         |
| `android-target` | `no-ort-target` + excel + tree-sitter + ocr + api + mcp (for x86_64-linux-android emulator)        |

## Build Profiles

- `release` — LTO thin, codegen-units=1, strip
- `profiling` — inherits release, retains debug info
- `kreuzberg-wasm` override: `opt-level="z"` (size-optimized)
- `sevenz-rust2`, `zip` override: `opt-level=2` (prevents SIGBUS on macOS ARM64)

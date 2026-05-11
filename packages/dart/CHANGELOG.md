# Changelog

## 5.0.0-rc.1

First Dart/Flutter release of the kreuzberg document intelligence library.

- Flutter FFI plugin (Android `arm64-v8a`/`x86_64`, iOS `arm64`/`arm64-sim`) backed by the kreuzberg Rust core via flutter_rust_bridge.
- Document extraction surface: `extractBytes`, `extractFile`, `extractFileSync`, `extractBytesSync`, batch variants, MIME detection (`detectMimeType`, `detectMimeTypeFromBytes`).
- Plugin lifecycle: `OcrBackend`, `PostProcessor`, `Validator`, `EmbeddingBackend`, `DocumentExtractor`, `Renderer` — each with register/unregister/clear/list helpers.
- Embedding presets: `listEmbeddingPresets`, `getEmbeddingPreset`, `embedTexts`.

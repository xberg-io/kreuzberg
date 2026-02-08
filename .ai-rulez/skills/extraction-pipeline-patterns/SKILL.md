---
name: extraction-pipeline-patterns
priority: critical
---

# Extraction Pipeline Patterns

**Kreuzberg's format detection -> extraction -> fallback orchestration for 56+ file formats**

## Core Pipeline Architecture

The extraction pipeline (`crates/kreuzberg/src/core/pipeline.rs`, `crates/kreuzberg/src/extraction/`) orchestrates:
1. **Format Detection** - MIME type inference + extension validation -> select appropriate extractor
2. **Intelligent Extraction** - Route to format-specific extractors (PDF, DOCX, Excel, HTML, images, archives, etc.)
3. **Fallback Strategies** - Password-protected PDFs, OCR for images, nested archive handling, corrupted file recovery
4. **Post-Processing Pipeline** - Validators, quality processing, chunking, custom hooks (see `core/pipeline.rs`)

## Format Detection Strategy

**Location**: `crates/kreuzberg/src/core/mime.rs`, `crates/kreuzberg/src/core/formats.rs`

Pattern: detect via magic bytes, validate extension alignment (prevent spoofing), route to extractor. Multiple extractors for same format -> choose highest confidence/specificity.

```rust
// Pseudocode: core/mime.rs
match (magic_bytes(content), extension) {
    (Some(fmt), Some(ext)) if aligned -> Ok(fmt),
    (Some(fmt), Some(ext)) if misaligned -> Err(FormatMismatch),
    (Some(fmt), None) -> Ok(fmt),  // magic bytes only
    (None, Some(ext)) -> Ok(from_extension(ext)),
    _ -> Err(UnknownFormat),
}
```

## Extraction Modules (56 Formats)

| Category | Extractors | Key Modules |
|----------|-----------|------------|
| **Office** | DOCX, XLSX, XLSM, XLSB, XLS, PPTX, ODP, ODS | `extraction/{docx,excel,pptx}.rs` |
| **PDF** | Standard + encrypted, password attempts | `pdf/` subdirectory (13 files) |
| **Images** | PNG, JPG, TIFF, WebP, JP2, SVG (OCR-enabled) | `extraction/image.rs` + `ocr/` |
| **Web** | HTML, XHTML, XML, SVG (DOM parsing) | `extraction/html.rs` (67KB - complex table handling) |
| **Email** | EML, MSG (headers, body, attachments, threading) | `extraction/email.rs` |
| **Archives** | ZIP, TAR, GZ, 7Z (recursive extraction) | `extraction/archive.rs` (31KB) |
| **Markdown** | MD, TXT, RST, Org Mode, RTF | `extraction/markdown.rs` |
| **Academic** | LaTeX, BibTeX, JATS, Jupyter, DocBook | `extraction/{structured,xml}.rs` |

## Extraction Dispatcher

```rust
// Pseudocode: extraction/mod.rs
let format = detect_format(source.bytes, source.extension);
let result = match format {
    Pdf -> extract_pdf(source, config),
    Docx -> extract_docx(source, config),
    Image -> extract_image_with_ocr_fallback(source, config),
    Archive -> extract_archive_recursive(source, config),
    _ -> extract_with_plugin(format, source, config),
};
run_pipeline(result, config)  // post-processing always runs
```

## Fallback Strategies

- **Password-Protected PDFs**: Try primary password -> secondary password list -> return `is_encrypted=true` in metadata on failure
- **OCR Fallback**: If image text extraction confidence < threshold, trigger OCR backend; return both results with scores
- **Nested Archives**: Recursive extraction with configurable depth limit; flatten or preserve hierarchy
- **Corrupted File Recovery**: Stream-based parsing, emit content up to error point, include error location in metadata

## Configuration Integration

**Location**: `crates/kreuzberg/src/core/config.rs`, `crates/kreuzberg/src/core/config_validation.rs`

`ExtractionConfig` holds format-specific configs (`pdf`, `image`, `html`, `office`), fallback orchestration (`fallback`), and post-processing (`postprocessor`, `chunking`, `keywords`). See struct definition in `config.rs`.

## Plugin System Integration

**Location**: `crates/kreuzberg/src/plugins/`

- **CustomExtractor**: Override built-in format extractors
- **PostProcessor**: Modify results after extraction (Early/Middle/Late stages)
- **Validator**: Fail-fast validation (e.g., minimum text length)
- **OCRBackend**: Swap OCR engine

Plugin registry loaded at startup, cached for zero-cost lookup.

## Feature Flag Strategy

**Location**: `Cargo.toml` (workspace), `crates/kreuzberg/Cargo.toml`, `FEATURE_MATRIX.md`

20+ features across 9 language bindings. Key feature groups:

| Group | Features | Notes |
|-------|----------|-------|
| OCR | `tesseract` (default), `tesseract-static`, `ocr-minimal` | Mutually exclusive recommendation |
| Formats | `pdf`, `pdf-minimal`, `office`, `office-minimal` | |
| AI/ML | `embeddings` (requires ONNX), `keywords-yake`, `keywords-rake`, `language-detection` | |
| Server | `api` (Axum), `mcp`, `tokio-runtime`, `lite-runtime` | |
| Bindings | `python-bindings`, `ruby-bindings`, `php-bindings`, `node-bindings`, `wasm` | |

Conditional compilation: modules gated with `#[cfg(feature = "...")]`. Runtime `validate_config()` warns if requested feature not compiled in.

### Feature Flag Critical Rules
1. **Never mix conflicting features** - e.g., `ocr-minimal` + `tesseract` should error at compile time
2. **Always provide feature diagnostics** - Config validation must warn if feature unavailable
3. **Default to maximum feature set** - Unless embedded/minimal specifically requested
4. **Test all feature combinations** - Matrix testing in CI catches regressions
5. **WASM incompatible** with embeddings, keywords, OCR

## Critical Rules

1. **Always use format detection** before routing to extractors (prevent confusion attacks)
2. **Stream-based parsing** for PDFs/archives to handle multi-GB files
3. **Post-pipeline is mandatory**: All extraction results flow through `run_pipeline()` for validators/hooks
4. **Plugin overrides are order-dependent**: Plugins registered first take priority
5. **Fallback timeouts**: Set reasonable OCR/archive extraction timeouts (config-driven)
6. **Metadata preservation**: Include format detection confidence, extraction method used, any fallbacks applied

## Related Skills

- **ocr-backend-management** - OCR engine selection and image preprocessing
- **chunking-embeddings** - Post-extraction text splitting with FastEmbed
- **api-server-mcp** - Axum endpoint for extraction pipeline exposure and MCP server

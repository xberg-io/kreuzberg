---
description: "format specific extraction"
priority: high
---

## priority: high

# Format-Specific Extraction Workflows

## Office XML (DOCX/PPTX/ODT)

```
ZIP archive → Security validation → XML parsing → Text + tables + metadata
```

1. `ZipBombValidator::new(limits).validate(&mut archive)?`
2. Extract XML files from archive (`word/document.xml`, `ppt/slides/*.xml`, `content.xml`)
3. Parse with `quick-xml::Reader` (streaming) + `DepthValidator` + `StringGrowthValidator`
4. Extract metadata via `crate::extraction::office_metadata::extract_metadata()`
5. See: `extractors/docx.rs`, `extractors/pptx.rs`, `extractors/odt.rs`

## PDF

```
Bytes → pdfium-render → Per-page text + OCR fallback → Tables → Metadata
```

1. `pdfium.create_document_from_bytes(content, None)?`
2. Check if needs OCR: `config.force_ocr || !has_searchable_text()`
3. Extract text per page, tables if `config.pages` enabled
4. Feature-gated: `#[cfg(feature = "pdf")]`
5. See: `extractors/pdf/mod.rs`

## Archives (ZIP/TAR/7z/GZIP)

```
Validate → Extract metadata → Extract plaintext files only
```

1. `ZipBombValidator` BEFORE any extraction
2. Extract metadata (file list, sizes)
3. Extract text content from plaintext files
4. Use `build_archive_result()` helper
5. See: `extractors/archive.rs`, `extraction/archive/*.rs`

## Structured Text (JSON/YAML/TOML/XML)

```
Detect format from MIME → Parse → Pretty-print → Metadata
```

Single `StructuredExtractor` handles multiple MIME types. Parse with format-specific library, pretty-print to text.
See: `extractors/structured.rs`

## Email (EML/MSG)

```
Parse headers → Extract body (text/html) → Process attachments
```

See: `extraction/email.rs`, `extractors/email.rs`

## Common Helpers

| Helper | Location | Purpose |
|--------|----------|---------|
| `office_metadata::extract_metadata()` | `extraction/office.rs` | Office XML metadata |
| `cells_to_markdown()` | `extraction/mod.rs` | Convert cell grid to GFM table |
| `build_archive_result()` | `extraction/archive/mod.rs` | Standard archive result |

## Adding a New Format

1. Add MIME type to `EXT_TO_MIME` in `core/mime.rs`
2. Create extractor implementing `DocumentExtractor` trait
3. Set `supported_mime_types()` and `priority()` (default: 50)
4. Register in `extractors/mod.rs` → `register_default_extractors()`
5. Feature-gate if optional: `#[cfg(feature = "my-format")]`
6. Apply security validators for user content
7. Add tests with fixture files
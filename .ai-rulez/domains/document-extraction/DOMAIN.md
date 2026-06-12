---
description: Document extraction pipeline architecture
---

- Pipeline: file input → MIME detection (magic bytes + extension) → extractor routing → extraction → post-processing → ExtractionResult
- Extractors are plugins implementing the Extractor trait: extract(&self, source: &ExtractionSource) → ExtractionResult
- Fallback chains: if primary extractor fails, try next in priority order (e.g., native PDF → Tesseract OCR → error)
- Cache-first: check extraction cache before running extractors, cache results keyed by content hash
- ExtractionResult contains: text content, metadata (page count, language, confidence), optional structured data (tables, images)
- Async-first: all extraction paths are async, use spawn_blocking for CPU-bound work (OCR, image processing)
- Memory limits: streaming for large files, configurable max file size, depth limits for nested archives
- Format coverage: 96 formats — PDF, DOCX, XLSX, PPTX, HTML, images (incl. HEIC/HEIF/AVIF), email (EML/MSG), archives, plain text

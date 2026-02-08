# Document Extraction Domain

## Purpose

Central hub for document intelligence: detects formats, routes to extractors, manages fallback chains, and coordinates extraction lifecycle through post-processing.

## Key Responsibilities

### 1. Multi-Format Detection
- MIME type identification via file extension, magic bytes, and content analysis
- Format classification: PDF, Office, Markdown, HTML, Images, binary
- Runtime detection independent of file extension
- Legacy format conversion (DOC/PPT to DOCX/PPTX)

### 2. Extractor Routing & Selection
- Priority-based plugin registry (default 50, custom > 50 to override, < 50 for fallback)
- Multiple extractors per MIME type with priority arbitration
- Fallback chain on extractor failure

### 3. Fallback Chains & Error Recovery
- Graceful degradation with fallback extraction strategies
- Batch processing continues on individual document failures
- Partial results with error metadata when complete extraction fails

### 4. Cache Integration
- Content-hash-based result caching with config-aware invalidation
- Sub-millisecond cached result retrieval

## Core Components

### Extract Entry Points (`core/extractor.rs`)
- `extract_file()` / `extract_bytes()` - Single document (async)
- `batch_extract_file()` / `batch_extract_bytes()` - Concurrent multi-document
- Synchronous wrappers for FFI and blocking contexts

### MIME Type System (`core/mime.rs`)
- Detection, routing, legacy format conversion coordination

### Error Handling & Telemetry
- OpenTelemetry spans with error classification (ValidationError, ParsingError, OCRError, MissingDependencyError)
- Sanitized telemetry (excludes full file paths)

## Document Extractor Trait

```rust
#[async_trait]
pub trait DocumentExtractor: Plugin {
    async fn extract_bytes(&self, content: &[u8], mime_type: &str, config: &ExtractionConfig)
        -> Result<ExtractionResult>;
    async fn extract_file(&self, path: &Path, mime_type: &str, config: &ExtractionConfig)
        -> Result<ExtractionResult>;
    fn supported_mime_types(&self) -> &[&str];
}
```

All extractors must be Send + Sync. Config-driven behavior. Unified ExtractionResult output.

### Built-in Extractors
- **PDF**: PDF/A, encrypted, complex layouts (pdfplumber, PyMuPDF, Docling, MinerU)
- **Office**: DOCX, XLSX, PPTX with layout/table preservation
- **HTML/Markdown**: Structured web content extraction
- **Image**: OCR pipeline orchestration
- **Text**: Plaintext with encoding detection

### Extraction Configuration
See: `crates/kreuzberg/src/core/config.rs` (ExtractionConfig struct with PdfConfig, OcrConfig, ChunkingConfig, EmbeddingConfig, etc.)

## Data Flow

### Extraction Pipeline
1. **Input** -> File path or byte array
2. **Cache Check** -> Return cached result if available
3. **MIME Detection** -> Identify document format
4. **Format Conversion** -> Convert legacy formats if needed
5. **Extractor Selection** -> Highest-priority matching extractor
6. **Extraction** -> Execute with config
7. **Error Handling** -> Try fallback extractors on failure
8. **Post-Processing** -> Document enhancement (keywords, entities)
9. **Caching** -> Store result
10. **Output** -> ExtractionResult

### Result Structure
See: `crates/kreuzberg/src/core/result.rs` (ExtractionResult: content, mime_type, metadata, tables, detected_languages, chunks, images, pages)

## Dependencies & Relationships

### Upstream
- **Kreuzberg Core**: Rust extraction library
- **LibreOffice**: Legacy format conversion
- **MIME Detection Library**: Format detection

### Downstream
- **OCR Integration Domain**: Image and embedded image processing
- **Plugin System Domain**: DocumentExtractor plugin management
- **Caching Layer**: Result storage

## Performance Characteristics

- **Single Document**: 50-500ms depending on format/size
- **Cached Lookup**: <1ms
- **Batch Processing**: Concurrent with configurable worker pool
- **Memory**: Linear with document size (streaming for large PDFs)

## Testing & Validation

- Format coverage for each major format (PDF, DOCX, XLSX, etc.)
- Fallback behavior with corrupted/malformed documents
- Benchmark extraction speeds for various document sizes
- Cache hit/invalidation correctness
- Custom extractor registration and priority selection

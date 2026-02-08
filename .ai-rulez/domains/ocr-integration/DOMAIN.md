# OCR Integration Domain

## Purpose

Manages OCR for document images and embedded images. Provides pluggable backend system (Tesseract, EasyOCR, PaddleOCR), image preprocessing, result caching, and hOCR parsing with table reconstruction.

## Key Responsibilities

### 1. Multiple OCR Backends
- **Backend Registry**: Pluggable registry of OCR implementations
- **Tesseract**: Native C bindings via `kreuzberg-tesseract` with PSM modes 0-13 for different text layouts (see Tesseract docs), multi-language support, hOCR output
- **Python OCR Backends**: EasyOCR/PaddleOCR via PyO3 with async execution (tokio blocking tasks), GIL management, configurable model loading
- **Backend Selection**: Based on image characteristics, language support, performance requirements, accuracy preferences

### 2. Image Preprocessing
- Format normalization (PNG, JPG, TIFF, WebP to standardized format)
- Resolution optimization: upscale <150 DPI, downsample extremely high-res
- Noise reduction, contrast enhancement (CLAHE), deskewing, binarization
- Region isolation for specialized handling (text blocks, tables)

### 3. OCR Result Processing & Caching
- Content-hash-based persistent cache
- hOCR to Markdown conversion with preserved layout
- Table reconstruction from hOCR bounding boxes
- Word-level confidence tracking, language detection
- Concurrent batch processing with resource pooling

### 4. Language Management
- Language pack validation and multi-language support
- Per-document language configuration
- Runtime installation hints for missing packs

## Core Components

### OCR Processor (`ocr/processor.rs`)
- `OcrProcessor::new()` - Initialize with optional cache config
- `process_image()` - Single image with TesseractConfig
- `batch_process_images()` - Concurrent batch processing

### OCR Backend Trait

```rust
#[async_trait]
pub trait OcrBackend: Send + Sync {
    async fn process_image(&self, image_bytes: &[u8], config: &TesseractConfig)
        -> Result<ExtractionResult>;
    fn supported_languages(&self) -> Vec<String>;
    fn name(&self) -> String;
}
```

### Tesseract Backend (`ocr/tesseract_backend.rs`)
Direct C FFI bindings, all PSM modes, hOCR output parsing, per-region configuration.

### hOCR Processing (`ocr/hocr.rs`)
Parses bounding boxes, confidence scores, formatting info (bold/italic to Markdown), preserves spatial layout for table reconstruction.

### Table Reconstruction (`ocr/table/mod.rs`)
- `reconstruct_table()` - hOCR positioned text to table structure
- `extract_words_from_tsv()` - Parse Tesseract TSV output
- `table_to_markdown()` - Format as Markdown tables

### OCR Caching (`ocr/cache.rs`)
Content-based hash keys, persistent storage (filesystem, Redis), statistics tracking, config-change invalidation.

### Language Registry (`ocr/language_registry.rs`)
Detect available packs, validate codes, provide installation hints.

### Configuration Types
See: `crates/kreuzberg/src/ocr/types.rs` (TesseractConfig: languages, PSMMode, OEM, preprocessing, confidence_threshold)

## Image Extraction Pipeline

1. **Image Detection** -> Format and size
2. **Preprocessing** -> Conditional transformations
3. **OCR Selection** -> Choose backend per config
4. **OCR Execution** -> Process and extract text
5. **Result Enhancement** -> Parse hOCR, reconstruct tables, detect languages
6. **Caching** -> Store result
7. **Return** -> Text and table results to document extraction pipeline

### Embedded Image Handling
Documents with embedded images (scanned PDFs, Office docs): images extracted, OCR'd, results incorporated into main content. See: `crates/kreuzberg/src/core/result.rs` (ExtractedImage: image_data, extracted_text, position, confidence)

### Configuration Integration
See: `crates/kreuzberg/src/core/config.rs` (OcrConfig: enabled, default_languages, cache_enabled, preprocessing, backend_priority)

### Python Plugin Interface
See: `crates/kreuzberg-py/src/plugins.rs` (PythonOcrBackend wraps Py<PyAny> with cached name/languages)
- GIL management for safe Python-Rust FFI
- Async execution via tokio::task::spawn_blocking
- Python exception handling with error translation

## Dependencies & Relationships

### Upstream
- **kreuzberg-tesseract**: Rust-C Tesseract bindings
- **image / imageproc**: Image manipulation
- **pyo3**: Python FFI for plugin backends

### Downstream
- **Document Extraction Domain**: Calls OCR for images
- **Plugin System Domain**: Manages Python OCR backends
- **Caching Layer**: Persistent OCR result storage

## Performance Characteristics

### Processing Times (per image)
- **Preprocessing**: 5-50ms | **Tesseract OCR**: 50-500ms | **hOCR Parsing**: <5ms | **Cache Lookup**: <1ms

### Memory
- Tesseract instance: ~50-100MB per process
- Preprocessing: proportional to image resolution

### Cache Efficiency
- Hit rate: 30-50% in production | Storage: ~5KB per result

## Testing & Validation

- Format coverage: PNG, JPG, TIFF, WebP
- Resolution handling: low/high DPI preprocessing
- Multi-language and PSM mode validation
- Table reconstruction accuracy on complex layouts
- Error handling for blurry/noisy/inverted images
- Cache consistency across process boundaries

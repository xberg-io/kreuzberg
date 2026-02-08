---
name: ocr-backend-management
priority: high
---

# OCR Backend Management

**Kreuzberg's pluggable OCR engine selection, image preprocessing, and table detection**

## OCR Architecture Overview

**Location**: `crates/kreuzberg/src/ocr/`, `crates/kreuzberg-tesseract/`, `crates/kreuzberg/src/extraction/image.rs`

| Backend | Language | Module | Features |
|---------|----------|--------|----------|
| **Tesseract** (Default) | Rust FFI binding | `kreuzberg-tesseract` crate | Multi-language, HOCR output, Tesseract 5.x |
| **EasyOCR** | Python-only | `kreuzberg-py` embedding | 80+ languages, GPU support |
| **PaddleOCR** | Python-only | `kreuzberg-py` embedding | Fast, Chinese/Japanese optimized |
| **Guten** | Node.js-only | `@kreuzberg/node` | Browser-compatible, lightweight |

### Module Structure
```
crates/kreuzberg/src/ocr/
├── mod.rs                 # OCR system orchestration
├── processor.rs           # Main OCR processing pipeline
├── tesseract_backend.rs   # Native Tesseract integration
├── language_registry.rs   # Language pack management
├── cache.rs              # OCR result caching
├── hocr.rs               # HOCR parsing
├── types.rs              # OCR configuration types
└── table/
    ├── detection.rs      # Table boundary detection
    └── reconstruction.rs # Structured table output
```

## Tesseract Configuration

**Location**: `crates/kreuzberg/src/ocr/tesseract_backend.rs`

Key config fields (see `TesseractConfig` struct):
- `tessdata_prefix` -- Custom Tesseract data dir
- `language_pack` -- e.g., `"eng"`, `"fra+deu"` for multi-language
- `oem` -- 0=Legacy, 1=Neural, 2=Both, 3=Default
- `psm` -- Page Segmentation Mode (0-13, see below)
- `dpi`, `quality_threshold`, `output_hocr`, `extract_tables`

### Tesseract PSM Mode Reference

| PSM | Mode | Use Case |
|-----|------|----------|
| 0 | OSD only | Detect orientation/script |
| 1 | Auto + OSD | General with orientation detection |
| 3 | Fully automatic (default) | Standard documents |
| 4 | Single column | Single-column text |
| 6 | Uniform block | Tables, forms |
| 7 | Single line | Receipts, captions |
| 8 | Single word | OCR on cropped regions |
| 11 | Sparse text | Scattered text on image |
| 13 | Raw line | No Tesseract page processing |

**Key pattern**: Lazy-initialize Tesseract instance per language pack (cached in `RwLock<HashMap<String, TesseractInstance>>`) to avoid repeated startup overhead.

## Image Preprocessing Pipeline

**Location**: `crates/kreuzberg/src/extraction/image.rs`

```
Input Image
    |
[1. Load & Validate] -> Check format, dimensions, color space
    |
[2. Preprocessing] -> Denoise, deskew, binarize (optional, config-driven)
    |
[3. Scale/DPI Adjustment] -> Resample to optimal DPI for OCR
    |
[4. Table Detection] -> Identify table regions for separate OCR
    |
[5. OCR Processing] -> Tesseract -> hOCR output with bounding boxes
    |
[6. Table Reconstruction] -> Structured table output (rows/cols)
    |
Output: ExtractionResult with text, tables, metadata (EXIF, dimensions)
```

Preprocessing steps (see `ImagePreprocessingConfig` in `extraction/image.rs`): denoise (bilateral filter), deskew (auto-rotate), binarize (Otsu or fixed threshold), DPI resample. Uses `image` crate + SIMD.

## Table Detection & HOCR

**Location**: `crates/kreuzberg/src/ocr/table/`, `crates/kreuzberg/src/ocr/hocr.rs`

Table detection: horizontal/vertical line detection -> grid validation -> per-cell OCR -> reconstruct as `TableData { headers, rows }`.

HOCR processing: parse Tesseract XML output for character-level bounding boxes, preserve reading order, cluster bboxes to identify table structures.

## Plugin System: Custom OCR Backends

**Location**: `crates/kreuzberg/src/plugins/ocr.rs`

```rust
pub trait OcrBackend: Send + Sync + 'static {
    async fn process(&self, image: &[u8], config: &OcrConfig) -> Result<OcrResult>;
    fn supported_languages(&self) -> Vec<String>;
    async fn health_check(&self) -> Result<()>;
}
```

Register custom backends (e.g., cloud OCR) via `OcrPluginRegistry::register(name, backend)` at startup.

## OCR Caching

**Location**: `crates/kreuzberg/src/ocr/cache.rs`

LRU cache keyed by `(image_hash, language, config_hash)` -> `OcrResult`. Pattern: `get_or_process()` checks cache first, processes on miss, stores result. Prevents re-OCR of identical images.

## Backend Selection Precedence

1. **Explicit config** - `config.image.ocr_backend = Some("aws-textract")`
2. **Plugin registered** - Check plugin registry
3. **Platform-specific default** - Python: EasyOCR, Node: Guten, Others: Tesseract
4. **Tesseract fallback** - Always available if installed
5. **Text-only** - Skip OCR if no backend available (log warning)

## Critical Rules

1. **Always validate language packs** before OCR (Tesseract installation check in health checks)
2. **Preprocess images** before OCR for 10-30% accuracy boost
3. **Cache OCR results** to avoid re-processing identical documents
4. **Table detection is optional** but recommended for document intelligence
5. **Timeout OCR operations** to prevent hanging on extremely large images (config: `ocr_timeout_secs`)
6. **Log confidence scores** in metadata for quality tracking
7. **Preserve EXIF data** from images (orientation, camera, GPS if present)

## Related Skills

- **extraction-pipeline-patterns** - OCR fallback integration in document extraction
- **chunking-embeddings** - Post-OCR text splitting and embedding generation
- **api-server-mcp** - OCR endpoint configuration and async processing

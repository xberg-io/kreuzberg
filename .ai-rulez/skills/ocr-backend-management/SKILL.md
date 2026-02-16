---
name: ocr-backend-management
priority: high
---

# OCR Backend Management

**Kreuzberg's pluggable OCR engine selection, image preprocessing, and table detection**

## OCR Architecture Overview

**Location**: `crates/kreuzberg/src/ocr/`, `crates/kreuzberg-tesseract/`, `crates/kreuzberg/src/extraction/image.rs`

Kreuzberg provides a pluggable OCR system with multiple backend implementations:

| Backend | Language | Module | Features |
|---------|----------|--------|----------|
| **Tesseract** (Default) | Rust FFI binding | `kreuzberg-tesseract` crate | Multi-language, HOCR output, Tesseract 5.x |
| **PaddleOCR** | Rust (ONNX Runtime) | `kreuzberg-paddle-ocr` crate | 80+ languages, 11 script families, auto model download |
| **EasyOCR** | Python-only | `kreuzberg-py` embedding | 80+ languages, GPU support |

## Core OCR Components

### 1. OCR Module Structure
```
crates/kreuzberg/src/ocr/
├── mod.rs                 # OCR system orchestration
├── processor.rs           # Main OCR processing pipeline
├── tesseract_backend.rs   # Native Tesseract integration (30KB)
├── language_registry.rs   # Language pack management (17KB)
├── cache.rs              # OCR result caching (15KB)
├── hocr.rs               # HOCR parsing (6KB)
├── types.rs              # OCR configuration types (13KB)
└── table/
    ├── detection.rs      # Table boundary detection
    └── reconstruction.rs # Structured table output
```

### 2. Tesseract Backend Integration

**Location**: `crates/kreuzberg/src/ocr/tesseract_backend.rs`

```rust
pub struct TesseractConfig {
    // Engine configuration
    pub tessdata_prefix: Option<String>,  // Custom Tesseract data dir
    pub language_pack: Option<String>,    // Language: "eng", "fra+deu", etc.
    pub oem: Option<OcrEngineMode>,      // 0=Legacy, 1=Neural, 2=Both, 3=Default

    // Processing options
    pub psm: Option<PageSegmentationMode>, // PSM 0-13 (see Tesseract docs)
    pub dpi: Option<u32>,                  // Image DPI for scaling
    pub quality_threshold: f32,            // Min confidence 0.0-1.0

    // Output
    pub output_hocr: bool,                 // Enable hOCR bounding boxes
    pub extract_tables: bool,              // Auto table detection
}
```

**Key Pattern**: Lazy-initialize Tesseract instance per language pack to avoid repeated startup overhead.

```rust
// Location: ocr/processor.rs
lazy_static! {
    static ref TESSERACT_INSTANCES: RwLock<HashMap<String, TesseractInstance>> = RwLock::new(HashMap::new());
}

async fn get_or_init_tesseract(language: &str) -> Result<TesseractInstance> {
    let mut instances = TESSERACT_INSTANCES.write().map_err(|e| OCRError::LockPoisoned)?;

    if let Some(instance) = instances.get(language) {
        Ok(instance.clone())
    } else {
        let instance = TesseractInstance::new(language)?;
        instances.insert(language.to_string(), instance.clone());
        Ok(instance)
    }
}
```

### 3. Language Pack Management

**Location**: `crates/kreuzberg/src/ocr/language_registry.rs` (17KB)

```rust
pub struct LanguageRegistry {
    available_languages: HashMap<String, LanguageMetadata>,
    cache: RwLock<HashMap<String, Vec<u8>>>,  // Cached language packs
}

impl LanguageRegistry {
    pub fn validate_language(&self, lang_code: &str) -> Result<LanguageMetadata> {
        // ISO 639-1/3 validation
        // Check if tessdata available for language
        // Return metadata: script, rtl, confidence_threshold
    }

    pub fn list_available(&self) -> Vec<AvailableLanguage> {
        // Return all installed language packs via tessdata check
    }

    pub fn download_language(&self, lang: &str) -> Result<()> {
        // Download missing language pack if available
        // Cache in TESSDATA_PREFIX
    }
}
```

**Supported Languages**: Tesseract ships with 100+ languages; Kreuzberg validates on startup.

## Image Preprocessing Pipeline

**Location**: `crates/kreuzberg/src/extraction/image.rs` (16KB)

```
Input Image
    ↓
[1. Load & Validate] → Check format, dimensions, color space
    ↓
[2. Preprocessing] → Denoise, deskew, binarize (optional, config-driven)
    ↓
[3. Scale/DPI Adjustment] → Resample to optimal DPI for OCR
    ↓
[4. Table Detection] → Identify table regions for separate OCR
    ↓
[5. OCR Processing] → Tesseract → hOCR output with bounding boxes
    ↓
[6. Table Reconstruction] → Structured table output (rows/cols)
    ↓
Output: ExtractionResult with text, tables, metadata (EXIF, dimensions)
```

### Preprocessing Configuration

```rust
pub struct ImagePreprocessingConfig {
    // Denoising
    pub denoise_enabled: bool,          // Bilateral filter or morphological
    pub denoise_strength: f32,          // 0.0-1.0 aggressiveness

    // Geometric
    pub deskew_enabled: bool,           // Auto-rotate skewed pages
    pub rotation_threshold_deg: f32,    // Min rotation angle to correct

    // Binarization
    pub binarize_enabled: bool,         // Convert to B&W (improves OCR)
    pub threshold_auto: bool,           // Otsu vs fixed threshold
    pub threshold_value: u8,            // If threshold_auto=false

    // Scaling
    pub target_dpi: u32,                // Resample to DPI (e.g., 300 DPI)
    pub preserve_aspect: bool,          // Don't distort
}
```

**Implementation Pattern**: Use `image` crate + SIMD operations for preprocessing.

```rust
// Location: extraction/image.rs
pub async fn preprocess_image(
    image_data: &[u8],
    config: &ImagePreprocessingConfig,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut img = image::load_from_memory(image_data)?;

    // Deskew if needed
    if config.deskew_enabled {
        img = deskew_image(img, config.rotation_threshold_deg)?;
    }

    // Denoise if needed
    if config.denoise_enabled {
        img = denoise_image(img, config.denoise_strength)?;
    }

    // Binarize if needed
    if config.binarize_enabled {
        img = binarize_image(img, config.threshold_auto, config.threshold_value)?;
    }

    // DPI adjustment
    if config.target_dpi > 0 {
        img = resample_to_dpi(img, config.target_dpi)?;
    }

    Ok(img)
}
```

## Table Detection & Reconstruction

**Location**: `crates/kreuzberg/src/ocr/table/`

### Table Detection Strategy

```rust
pub struct TableDetectionConfig {
    pub enabled: bool,                 // Enable table detection
    pub min_cells: u32,               // Minimum cell count to consider table
    pub confidence_threshold: f32,    // Min confidence in table structure
    pub line_thickness_threshold: u32, // Min pixels for table borders
    pub merge_adjacent_cells: bool,   // Merge cells with same span
}
```

**Pattern**: Detect table regions via:
1. Horizontal/vertical line detection (Hough transform or morphological operations)
2. Grid structure validation (rows × columns consistency)
3. Cell content extraction (OCR each cell)
4. Reconstruct as `TableData { headers: Vec<String>, rows: Vec<Vec<String>> }`

### HOCR Processing

**Location**: `crates/kreuzberg/src/ocr/hocr.rs`

Tesseract's hOCR output contains XML with bounding boxes:
```xml
<span class="ocr_word" id="word_1_1" title="bbox 100 200 250 230">word</span>
```

Kreuzberg parses hOCR to:
- Extract text with character-level bounding boxes
- Preserve reading order (top-left → bottom-right)
- Identify table structures via bbox clustering
- Return as `TextWithBoundingBoxes { text: String, boxes: Vec<BBox> }`

## Plugin System: Custom OCR Backends

**Location**: `crates/kreuzberg/src/plugins/ocr.rs`

```rust
pub trait OcrBackend: Send + Sync + 'static {
    /// Process image and return text with optional HOCR/tables
    async fn process(
        &self,
        image: &[u8],
        config: &OcrConfig,
    ) -> Result<OcrResult>;

    /// List supported languages
    fn supported_languages(&self) -> Vec<String>;

    /// Health check (verify engine is available)
    async fn health_check(&self) -> Result<()>;
}

pub struct OcrPluginRegistry {
    backends: HashMap<String, Arc<dyn OcrBackend>>,
}

impl OcrPluginRegistry {
    pub fn register(&mut self, name: &str, backend: Arc<dyn OcrBackend>) {
        self.backends.insert(name.to_string(), backend);
    }

    pub fn get_backend(&self, name: &str) -> Result<Arc<dyn OcrBackend>> {
        self.backends.get(name).cloned()
            .ok_or_else(|| OCRError::BackendNotFound(name.to_string()))
    }
}
```

**Usage Pattern**: Register custom backend (e.g., cloud-based OCR, proprietary engine) at startup.

```rust
// Example: Custom AWS Textract backend
let textract_backend = Arc::new(AwsTextractBackend::new(credentials)?);
plugins::ocr::register_backend("aws-textract", textract_backend)?;

// Use via config
let config = ExtractionConfig {
    image: Some(ImageConfig {
        ocr_backend: Some("aws-textract".to_string()),
        ..Default::default()
    }),
    ..Default::default()
};
```

## Language Detection Integration

**Location**: `crates/kreuzberg/src/language_detection/`

Before OCR, auto-detect document language:

```rust
pub struct LanguageDetectionConfig {
    pub enabled: bool,                 // Auto-detect language
    pub confidence_threshold: f32,    // Min confidence 0.0-1.0
    pub fallback_languages: Vec<String>, // If detection fails
}

// Usage in image extraction
let detected_lang = detect_language(&image_text)?;
let ocr_config = ImageConfig {
    ocr_language: Some(detected_lang),
    ..Default::default()
};
```

## Performance Optimization

### Caching Strategy

**Location**: `crates/kreuzberg/src/ocr/cache.rs`

```rust
pub struct OcrResultCache {
    // LRU cache: (image_hash, language, config_hash) → OcrResult
    cache: Arc<RwLock<LruCache<CacheKey, OcrResult>>>,
}

impl OcrResultCache {
    pub async fn get_or_process(
        &self,
        image: &[u8],
        language: &str,
        config: &OcrConfig,
    ) -> Result<OcrResult> {
        let key = CacheKey::new(image, language, config);

        // Check cache
        {
            let cache = self.cache.read()?;
            if let Some(result) = cache.get(&key) {
                return Ok(result.clone());
            }
        }

        // Cache miss: process
        let result = process_with_tesseract(image, language, config).await?;
        self.cache.write()?.put(key, result.clone());

        Ok(result)
    }
}
```

**Cache Key**: Hash of image + language + config to avoid false hits.

### Batch OCR Processing

```rust
// Process multiple images in parallel
pub async fn batch_ocr(
    images: Vec<(String, Vec<u8>)>,  // (file_id, image_data)
    config: &OcrConfig,
) -> Result<Vec<(String, OcrResult)>> {
    let tasks: Vec<_> = images
        .into_iter()
        .map(|(id, data)| tokio::spawn(process_image(id, data, config)))
        .collect();

    futures::future::try_join_all(tasks).await?
}
```

## Configuration Priority (Feature Matrix)

**Location**: `crates/kreuzberg/src/core/config.rs`

OCR backend selection precedence:
1. **Explicit config** - `config.image.ocr_backend = Some("aws-textract")`
2. **Plugin registered** - Check if backend registered in plugins registry
3. **Platform-specific default** - Python: EasyOCR, Others: Tesseract
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
- **feature-flag-strategy** - Conditional OCR backend compilation (Tesseract, EasyOCR, PaddleOCR)
- **api-server-patterns** - OCR endpoint configuration and async processing

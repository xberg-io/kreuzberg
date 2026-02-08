# Plugin System Domain

## Purpose

Core extensibility architecture for Kreuzberg. Manages plugin discovery, lifecycle, trait-based registries, and priority-based selection for custom document extractors, OCR backends, post-processors, and validators.

## Key Responsibilities

### 1. Plugin Architecture
- **Base Plugin Trait**: Common interface (name, version, initialize, shutdown) with Send + Sync
- **Type-Specific Traits**: DocumentExtractor, OcrBackend, PostProcessor, Validator
- **Lifecycle Methods**: Initialize/shutdown hooks for resource management

### 2. Plugin Discovery
- **Static Registration**: Direct Rust plugin instantiation and registration
- **Python Plugin Discovery**: Module path scanning, class detection, protocol validation, GIL management
- **Plugin Validation**: Verify trait implementation before registration

### 3. Priority Selection System
- Priority 0-255: higher wins. Default 50. Custom overrides > 50, fallbacks < 50
- MIME type arbitration: highest-priority plugin selected, fallback chain on failure
- Capability-based selection considering languages, dependencies, performance

### 4. Registry Management
- Separate registries per type: DocumentExtractorRegistry, OcrBackendRegistry, PostProcessorRegistry, ValidatorRegistry
- Thread-safe via RwLock, MIME type indexing for O(log n) lookup
- Dynamic registration/unregistration with optional event hooks

## Core Components

### Plugin Trait System (`plugins/mod.rs`)

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> String;
    fn initialize(&self) -> Result<()>;
    fn shutdown(&self) -> Result<()>;
}
```

### Document Extractor (`plugins/extractor.rs`)

```rust
#[async_trait]
pub trait DocumentExtractor: Plugin {
    async fn extract_bytes(&self, content: &[u8], mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult>;
    async fn extract_file(&self, path: &Path, mime_type: &str, config: &ExtractionConfig) -> Result<ExtractionResult>;
    fn supported_mime_types(&self) -> &[&str];
    fn priority(&self) -> u8 { 50 }
}
```

### OCR Backend (`plugins/ocr.rs`)

```rust
#[async_trait]
pub trait OcrBackend: Plugin {
    async fn process_image(&self, image_bytes: &[u8], config: &TesseractConfig) -> Result<ExtractionResult>;
    fn supported_languages(&self) -> Vec<String>;
    fn capabilities(&self) -> OcrCapabilities;
}
```

### Post-Processor (`plugins/postprocessor.rs`)

```rust
#[async_trait]
pub trait PostProcessor: Plugin {
    async fn process(&self, result: &mut ExtractionResult, config: &PostProcessorConfig) -> Result<()>;
    fn supported_mime_types(&self) -> &[&str];
    fn priority(&self) -> u8 { 50 }
}
```

### Validator (`plugins/validator.rs`)

```rust
#[async_trait]
pub trait Validator: Plugin {
    async fn validate(&self, result: &ExtractionResult) -> Result<ValidationReport>;
    fn validation_type(&self) -> ValidationType;
}
```

### Python Plugin FFI Bridge (`crates/kreuzberg-py/src/plugins.rs`)

```python
class MyCustomExtractor:
    def name(self) -> str: return "my-extractor"
    async def extract_bytes(self, content: bytes, mime_type: str, config) -> ExtractionResult: ...
    def supported_mime_types(self) -> list[str]: return ["application/custom"]
```

Async method support with GIL management, exception handling, error translation.

### Plugin Registry
See: `crates/kreuzberg/src/plugins/registry.rs` (DocumentExtractorRegistry: Arc<RwLock<Vec<Arc<dyn DocumentExtractor>>>> with MIME type index, register/unregister/get_for_mime/list_all/clear)

## Integration with Kreuzberg Architecture

### Document Extraction Pipeline
```
1. Detect MIME type
2. Query registry for matching plugins (sorted by priority)
3. Iterate attempting extraction; success -> return, failure -> next plugin
```

### OCR Pipeline Integration
```
1. Receive image + config -> query capable backends -> sort by priority -> execute -> cache
```

### Post-Processing Chain
Sequential execution in priority order. See: `crates/kreuzberg/src/core/config.rs` (PostProcessorConfig, PostProcessorSpec)

### Python Plugin Registration
```rust
pub fn register_ocr_backend(name: String, python_obj: Py<PyAny>, priority: u8) -> PyResult<()>;
pub fn register_post_processor(name: String, python_obj: Py<PyAny>, priority: u8) -> PyResult<()>;
```

## GIL Management for Python Plugins

### Critical GIL Patterns Used

1. **Temporary GIL Acquisition** (Python::attach)
   ```rust
   Python::attach(|py| {
       let result = python_obj.bind(py).call_method0("name")?;
       result.extract::<String>()
   })
   ```
   Use for quick operations (reading attributes, simple calls)

2. **GIL Release During Expensive Operations** (py.detach)
   ```rust
   py.detach(|| {
       let registry = get_registry();
       registry.write()?.register(backend)
   })
   ```
   Use for I/O, lock acquisition, expensive computation

3. **Async Python Calls** (tokio::task::spawn_blocking)
   ```rust
   let python_obj = Python::attach(|py| python_obj.clone_ref(py));
   tokio::task::spawn_blocking(move || {
       Python::attach(|py| obj.bind(py).call_method1("process_image", (bytes, config)))
   }).await?
   ```
   Use for async trait implementations

4. **Caching to Minimize GIL Acquisitions**
   Cache frequently-accessed Python data (name, supported_languages) in Rust fields.

## Data Flow

### Plugin Registration
1. Instantiate plugin (Rust or Python) -> validate traits -> find registry -> assign priority -> index update -> initialize

### Plugin Selection & Execution
1. Query by capability -> sort by priority -> iterate (fallback chain) -> execute -> cache result

## Dependencies & Relationships

### Upstream
- **Rust Core**: Base plugin traits and registries
- **PyO3**: Python FFI
- **async-trait**: Async trait support

### Downstream
- **Document Extraction Domain**: Uses DocumentExtractor plugins
- **OCR Integration Domain**: Uses OcrBackend plugins
- **Post-Processing / Validation**: Uses PostProcessor and Validator plugins

## Performance Characteristics

- **Registration**: O(1) Rust, O(n) Python discovery
- **Selection**: O(log n) with indexed MIME lookup
- **GIL Overhead**: ~5-55us per Python call (mitigated by caching)

## Testing & Validation

- Plugin interface compliance and priority-based arbitration
- Fallback chains and error handling in registration/execution
- Python FFI correctness and GIL release during expensive ops
- Thread safety under concurrent access
- Plugin selection overhead benchmarks

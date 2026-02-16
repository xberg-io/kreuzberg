```rust title="Rust"
use kreuzberg::vllm_ocr::VllmOcrBackend;
use kreuzberg::plugins::register_ocr_backend;
use std::sync::Arc;

// Custom vLLM configuration
let backend = VllmOcrBackend::builder()
    .endpoint("http://my-gpu-server:8000")
    .model("zai-org/GLM-OCR")
    .api_key("my-api-key") // optional
    .build();

register_ocr_backend(Arc::new(backend)).unwrap();
```

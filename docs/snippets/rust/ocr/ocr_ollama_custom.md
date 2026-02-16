```rust title="Rust"
use kreuzberg::ollama_ocr::OllamaOcrBackend;
use kreuzberg::plugins::register_ocr_backend;
use std::sync::Arc;

// Custom Ollama configuration
let backend = OllamaOcrBackend::builder()
    .endpoint("http://my-ollama-server:11434")
    .model("glm-ocr")
    .prompt("Extract all text from this image verbatim.")
    .build();

register_ocr_backend(Arc::new(backend)).unwrap();
```

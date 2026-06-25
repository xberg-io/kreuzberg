```rust title="Rust"
use xberg::plugins::registry::get_document_extractor_registry;

let registry = get_document_extractor_registry();
registry.remove("custom-json-extractor")?;
```

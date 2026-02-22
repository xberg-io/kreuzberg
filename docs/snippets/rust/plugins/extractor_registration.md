```rust title="Rust"
use kreuzberg::plugins::registry::get_document_extractor_registry;
use std::sync::Arc;

fn register_custom_extractor() -> kreuzberg::Result<()> {
    let extractor = Arc::new(CustomJsonExtractor);
    let registry = get_document_extractor_registry();
    registry.write().unwrap().register(extractor)?;
    Ok(())
}
```

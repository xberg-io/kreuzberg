```rust title="Rust"
use xberg::plugins::registry::get_document_extractor_registry;
use std::sync::Arc;

fn register_custom_extractor() -> xberg::Result<()> {
    let extractor = Arc::new(CustomJsonExtractor);
    let registry = get_document_extractor_registry();
    registry.write().unwrap().register(extractor)?;
    Ok(())
}
```

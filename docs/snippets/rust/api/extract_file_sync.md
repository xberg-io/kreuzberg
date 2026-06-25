```rust title="Rust"
use xberg::{extract_file_sync, ExtractionConfig};

fn main() -> xberg::Result<()> {
    let config = ExtractionConfig::default();
    let result = extract_file_sync("document.pdf", None, &config)?;

    println!("{}", result.content);
    println!("MIME type: {}", result.mime_type);
    println!("Tables: {}", result.tables.len());
    Ok(())
}
```

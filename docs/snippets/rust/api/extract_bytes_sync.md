```rust title="Rust"
use xberg::{extract_bytes_sync, ExtractionConfig};

fn main() -> xberg::Result<()> {
    let content = std::fs::read("document.pdf")?;
    let config = ExtractionConfig::default();
    let result = extract_bytes_sync(&content, "application/pdf", &config)?;

    println!("{}", result.content);
    println!("Tables: {}", result.tables.len());
    Ok(())
}
```

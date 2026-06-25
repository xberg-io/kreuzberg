```rust title="Rust"
use xberg::{extract_bytes, ExtractionConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let content = tokio::fs::read("document.pdf").await?;
    let config = ExtractionConfig::default();
    let result = extract_bytes(&content, "application/pdf", &config).await?;

    println!("{}", result.content);
    println!("Tables: {}", result.tables.len());
    Ok(())
}
```

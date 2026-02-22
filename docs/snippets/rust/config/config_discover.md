```rust title="Rust"
use kreuzberg::{extract_file, ExtractionConfig};

#[tokio::main]
async fn main() -> kreuzberg::Result<()> {
    let config = ExtractionConfig::discover()?.unwrap_or_default();
    let result = extract_file("document.pdf", None, &config).await?;
    println!("{}", result.content);
    Ok(())
}
```

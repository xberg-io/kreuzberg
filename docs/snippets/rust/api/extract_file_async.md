```rust title="Rust"
use xberg::{extract_file, ExtractionConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig::default();
    let result = extract_file("document.pdf", None::<&str>, &config).await?;

    println!("{}", result.content);
    println!("MIME type: {}", result.mime_type);
    println!("Tables: {}", result.tables.len());
    Ok(())
}
```

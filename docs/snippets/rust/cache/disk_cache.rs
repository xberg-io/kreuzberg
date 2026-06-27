```rust title="disk_cache.rs"
use xberg::{extract, ExtractionConfig, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let path = std::env::args()
        .skip(1)
        .find(|a| !a.is_empty() && a != "--")
        .unwrap_or_else(|| "document.pdf".to_string());

    // Enable caching (default: true). The Rust crate uses an internal disk cache.
    let config = ExtractionConfig {
        use_cache: true,
        ..Default::default()
    };

    println!("First extraction (will be cached)...");
    let output1 = extract(ExtractInput::from_uri(&path), &config).await?;
    println!("  - Content length: {}", output1.results[0].content.len());

    println!("\nSecond extraction (from cache when available)...");
    let output2 = extract(ExtractInput::from_uri(&path), &config).await?;
    println!("  - Content length: {}", output2.results[0].content.len());

    println!("\nResults are identical: {}", output1.results[0].content == output2.results[0].content);

    Ok(())
}
```

```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        enable_quality_processing: true,
        use_cache: true,
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    let result = &output.results[0];
    println!("Quality score: {}", result.quality_score);
    println!("Processing time: {:?}", result.processing_time);
    Ok(())
}
```

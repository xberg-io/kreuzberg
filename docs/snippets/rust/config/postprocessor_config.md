```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, PostProcessorConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        postprocessor: Some(PostProcessorConfig {
            enabled: true,
            enabled_processors: Some(vec![
                "whitespace_normalizer".to_string(),
                "unicode_normalizer".to_string(),
            ]),
            disabled_processors: None,
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("Processed content: {}", output.results[0].content);
    Ok(())
}
```

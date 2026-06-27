```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, LanguageDetectionConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        language_detection: Some(LanguageDetectionConfig {
            enabled: true,
            min_confidence: 0.8,
            detect_multiple: true,
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    let result = &output.results[0];
    println!("Detected language: {}", result.language);
    println!("Confidence: {}", result.language_confidence);
    Ok(())
}
```

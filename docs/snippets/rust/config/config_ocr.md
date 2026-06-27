```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, OcrConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng".to_string(),
            tesseract_config: None,
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("scanned.pdf"), &config).await?;
    println!("Content length: {}", output.results[0].content.len());
    println!("Tables detected: {}", output.results[0].tables.len());
    Ok(())
}
```

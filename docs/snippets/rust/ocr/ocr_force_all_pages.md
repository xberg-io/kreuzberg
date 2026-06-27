```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, OcrConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            ..Default::default()
        }),
        force_ocr: true,
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("{}", output.results[0].content);
    Ok(())
}
```

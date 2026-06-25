```rust title="Rust"
use xberg::{extract_file, ExtractionConfig, OcrConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "easyocr".to_string(),
            language: "en".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file("document.pdf", None, &config).await?;
    println!("Extracted text: {}", result.content);
    Ok(())
}
```

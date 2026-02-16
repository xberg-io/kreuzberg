```rust title="Rust"
use kreuzberg::{extract_file, ExtractionConfig, OcrConfig};

#[tokio::main]
async fn main() -> kreuzberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "ollama".to_string(),
            language: "eng".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file("scanned.pdf", None, &config).await?;
    println!("{}", result.content);
    Ok(())
}
```

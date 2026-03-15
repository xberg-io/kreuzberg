```rust title="Rust"
use kreuzberg::{extract_file, ExtractionConfig, OcrConfig};

#[tokio::main]
async fn main() -> kreuzberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "paddleocr".to_string(),
            language: "en".to_string(),
            paddle_ocr_config: Some(serde_json::json!({
                "model_tier": "mobile"
            })),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file("document.pdf", None, &config).await?;
    println!("Extracted text: {}", result.content);
    Ok(())
}
```

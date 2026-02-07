```rust title="Rust"
use kreuzberg::{extract_file, ExtractionConfig, OcrConfig};

#[tokio::main]
async fn main() -> kreuzberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "paddleocr".to_string(),
            language: "en".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file("scanned.pdf", None, &config).await?;

    if let Some(elements) = &result.ocr_elements {
        for element in elements {
            println!("Text: {}", element.text);
            println!("Confidence: {:.2}", element.confidence.recognition);
            println!("Geometry: {:?}", element.geometry);
            if let Some(rotation) = &element.rotation {
                println!("Rotation: {}°", rotation.angle);
            }
            println!();
        }
    }
    Ok(())
}
```

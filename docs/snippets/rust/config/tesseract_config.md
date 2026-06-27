```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, OcrConfig};
use xberg::types::TesseractConfig;

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng+deu".to_string(),
            tesseract_config: Some(TesseractConfig {
                psm: Some(6),
                oem: Some(3),
                ..Default::default()
            }),
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("scanned.pdf"), &config).await?;
    println!("OCR text: {}", output.results[0].content);
    Ok(())
}
```

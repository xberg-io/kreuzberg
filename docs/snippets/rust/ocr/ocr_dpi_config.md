```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, OcrConfig, PdfConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            ..Default::default()
        }),
        pdf_options: Some(PdfConfig {
            dpi: Some(300),
            ..Default::default()
        }),
        ..Default::default()
    };

    let _output = extract(ExtractInput::from_uri("scanned.pdf"), &config).await?;
    Ok(())
}
```

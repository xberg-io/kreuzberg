```rust title="Rust"
use xberg::{extract_file_sync, ExtractionConfig, OcrConfig};

fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = extract_file_sync("scanned.pdf", None, &config)?;
    println!("{}", result.content);
    Ok(())
}
```

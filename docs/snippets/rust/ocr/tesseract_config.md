```rust title="Rust"
use xberg::{ExtractionConfig, OcrConfig, TesseractConfig};

fn main() {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            language: "eng+fra+deu".to_string(),
            tesseract_config: Some(TesseractConfig {
                psm: 6,
                oem: 1,
                min_confidence: 0.8,
                tessedit_char_whitelist: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 .,!?".to_string(),
                enable_table_detection: true,
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    println!("{:?}", config.ocr);
}
```

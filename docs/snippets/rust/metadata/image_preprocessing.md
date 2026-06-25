```rust title="Rust"
use xberg::{ExtractionConfig, ImagePreprocessingConfig, OcrConfig, TesseractConfig};

fn main() {
    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            tesseract_config: Some(TesseractConfig {
                preprocessing: Some(ImagePreprocessingConfig {
                    target_dpi: 300,
                    denoise: true,
                    deskew: true,
                    contrast_enhance: true,
                    binarization_method: "otsu".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    println!("{:?}", config.ocr);
}
```

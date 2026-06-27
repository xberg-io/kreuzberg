```rust title="Rust"
use xberg::{
    extract, ExtractionConfig, ExtractInput, ImagePreprocessingConfig, OcrConfig, TesseractConfig,
};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let preprocessing = ImagePreprocessingConfig {
        target_dpi: 300,
        denoise: true,
        deskew: true,
        contrast_enhance: true,
        binarization_method: "otsu".to_string(),
        ..Default::default()
    };

    let config = ExtractionConfig {
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng".to_string(),
            tesseract_config: Some(TesseractConfig {
                preprocessing: Some(preprocessing),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("content length: {}", output.results[0].content.len());
    Ok(())
}
```

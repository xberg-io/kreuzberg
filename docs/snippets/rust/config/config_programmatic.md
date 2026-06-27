```rust title="Rust"
use xberg::{extract, ChunkingConfig, ExtractionConfig, ExtractInput, OcrConfig, TesseractConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ExtractionConfig {
        use_cache: true,
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: "eng+deu".to_string(),
            tesseract_config: Some(TesseractConfig {
                psm: 6,
                ..Default::default()
            }),
            ..Default::default()
        }),
        chunking: Some(ChunkingConfig {
            max_characters: 1000,
            overlap: 200,
            ..Default::default()
        }),
        enable_quality_processing: true,
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("Content length: {}", output.results[0].content.len());
    Ok(())
}
```

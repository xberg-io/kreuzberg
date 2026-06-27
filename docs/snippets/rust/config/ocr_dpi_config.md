```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, ImageExtractionConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            target_dpi: 300,
            max_image_dimension: 4096,
            auto_adjust_dpi: true,
            min_dpi: 150,
            max_dpi: 600,
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("Extracted images: {}", output.results[0].images.len());
    Ok(())
}
```

```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, ImageExtractionConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        images: Some(ImageExtractionConfig {
            extract_images: true,
            target_dpi: 200,
            max_image_dimension: 2048,
            inject_placeholders: true, // set to false to extract images without markdown references
            auto_adjust_dpi: true,
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("content length: {}", output.results[0].content.len());
    Ok(())
}
```

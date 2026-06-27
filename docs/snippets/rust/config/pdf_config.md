```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, PdfConfig, HierarchyConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        pdf_options: Some(PdfConfig {
            extract_images: true,
            passwords: Some(vec!["password123".to_string()]),
            extract_metadata: true,
            hierarchy: Some(HierarchyConfig::default()),
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("encrypted.pdf"), &config).await?;
    let result = &output.results[0];
    println!("Title: {:?}", result.metadata.title);
    println!("Authors: {:?}", result.metadata.authors);
    Ok(())
}
```

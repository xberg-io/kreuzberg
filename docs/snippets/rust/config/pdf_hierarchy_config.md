```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, PdfConfig, HierarchyConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        pdf_options: Some(PdfConfig {
            hierarchy: Some(HierarchyConfig {
                enabled: true,
                detection_threshold: Some(0.75),
                ocr_coverage_threshold: Some(0.8),
                min_level: Some(1),
                max_level: Some(5),
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("Hierarchy levels: {}", output.results[0].hierarchy.len());
    Ok(())
}
```

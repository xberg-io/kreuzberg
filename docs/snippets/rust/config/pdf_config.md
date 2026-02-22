```rust title="Rust"
use kreuzberg::{extract_file_sync, ExtractionConfig, PdfConfig, HierarchyConfig};

fn main() -> kreuzberg::Result<()> {
    let config = ExtractionConfig {
        pdf_options: Some(PdfConfig {
            extract_images: true,
            passwords: Some(vec!["password123".to_string()]),
            extract_metadata: true,
            hierarchy: Some(HierarchyConfig::default()),
        }),
        ..Default::default()
    };

    let result = extract_file_sync("encrypted.pdf", None, &config)?;
    println!("Title: {:?}", result.metadata.title);
    println!("Authors: {:?}", result.metadata.authors);
    Ok(())
}
```

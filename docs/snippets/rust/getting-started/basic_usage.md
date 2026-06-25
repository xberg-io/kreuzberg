```rust title="Rust"
use xberg::{extract_file_sync, ExtractionConfig};

fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        use_cache: true,
        enable_quality_processing: true,
        ..Default::default()
    };

    let result = extract_file_sync("document.pdf", None, &config)?;
    println!("{}", result.content);
    println!("MIME Type: {}", result.mime_type);
    Ok(())
}
```

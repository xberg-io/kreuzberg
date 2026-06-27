```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig::default();
    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    let result = &output.results[0];

    let Some(pages) = &result.metadata.pages else {
        return Ok(());
    };
    let Some(boundaries) = &pages.boundaries else {
        return Ok(());
    };

    for boundary in boundaries.iter().take(3) {
        let page_text = &result.content[boundary.byte_start..boundary.byte_end];
        let preview_end = 100.min(page_text.len());

        println!("Page {}:", boundary.page_number);
        println!("  Byte range: {}-{}", boundary.byte_start, boundary.byte_end);
        println!("  Preview: {}...", &page_text[..preview_end]);
    }

    Ok(())
}
```

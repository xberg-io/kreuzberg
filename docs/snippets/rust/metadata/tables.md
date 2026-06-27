```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let output = extract(ExtractInput::from_uri("document.pdf"), &ExtractionConfig::default()).await?;
    let result = &output.results[0];

    for table in &result.tables {
        println!("Table with {} rows", table.cells.len());
        println!("{}", table.markdown);

        for row in &table.cells {
            println!("{:?}", row);
        }
    }
    Ok(())
}
```

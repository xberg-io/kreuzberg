```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig::discover()?.unwrap_or_default();
    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("{}", output.results[0].content);
    Ok(())
}
```

```rust title="Rust"
use xberg::{extract, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let output = extract(ExtractInput::from_uri("document.pdf"), &Default::default()).await?;
    println!("Extraction successful: {}", !output.results[0].content.is_empty());
    println!("Content length: {} characters", output.results[0].content.len());
    Ok(())
}
```

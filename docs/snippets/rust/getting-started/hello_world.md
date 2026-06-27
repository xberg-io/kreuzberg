```rust title="Rust"
use xberg::{extract, ExtractInput};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let output = extract(ExtractInput::from_uri("document.pdf"), &Default::default()).await?;
    println!("{}", output.results[0].content);
    Ok(())
}
```

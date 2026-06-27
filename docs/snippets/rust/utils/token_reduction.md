```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, TokenReductionOptions};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        token_reduction: Some(TokenReductionOptions {
            mode: "moderate".to_string(),
            preserve_important_words: true,
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("Content length: {}", output.results[0].content.len());
    Ok(())
}
```

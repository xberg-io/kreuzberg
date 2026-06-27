```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, TokenReductionConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        token_reduction: Some(TokenReductionConfig {
            mode: "moderate".to_string(),
            preserve_important_words: true,
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    let result = &output.results[0];
    println!("Original tokens: {}", result.token_count);
    println!("Reduced content: {}", result.content);
    Ok(())
}
```

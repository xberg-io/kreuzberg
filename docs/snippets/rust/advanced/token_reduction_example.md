```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, TokenReductionConfig};

let config = ExtractionConfig {
    token_reduction: Some(TokenReductionConfig {
        mode: "moderate".to_string(),
        preserve_markdown: true,
        ..Default::default()
    }),
    ..Default::default()
};

let output = extract(ExtractInput::from_uri("verbose_document.pdf"), &config).await?;
let result = &output.results[0];

if let Some(original) = result.original_token_count {
    println!("Original tokens: {}", original);
}
if let Some(reduced) = result.reduced_token_count {
    println!("Reduced tokens: {}", reduced);
}
```

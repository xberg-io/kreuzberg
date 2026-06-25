```rust title="Rust"
use xberg::{ExtractionConfig, TokenReductionConfig};

fn main() {
    let config = ExtractionConfig {
        token_reduction: Some(TokenReductionConfig {
            mode: "moderate".to_string(),
            preserve_important_words: true,
        }),
        ..Default::default()
    };
    println!("{:?}", config.token_reduction);
}
```

```rust title="Rust"
use xberg::{ChunkingConfig, EmbeddingConfig, EmbeddingModelType, ExtractionConfig};

fn main() {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 1500,
            overlap: 200,
            embedding: Some(EmbeddingConfig {
                model: EmbeddingModelType::Preset {
                    name: "text-embedding-all-minilm-l6-v2".to_string(),
                },
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    println!("{:?}", config.chunking);
}
```

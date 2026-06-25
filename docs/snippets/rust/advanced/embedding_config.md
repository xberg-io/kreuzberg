```rust title="Rust"
use xberg::{ChunkingConfig, EmbeddingConfig, EmbeddingModelType, ExtractionConfig};

fn main() {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 1000,
            embedding: Some(EmbeddingConfig {
                model: EmbeddingModelType::Preset {
                    name: "all-mpnet-base-v2".to_string(),
                },
                batch_size: 16,
                normalize: true,
                show_download_progress: true,
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    println!("{:?}", config.chunking);
}
```

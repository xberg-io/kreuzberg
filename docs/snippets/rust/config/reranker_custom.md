```rust title="Rust"
use xberg::{RerankerConfig, RerankerModelType};

let config = RerankerConfig {
    model: RerankerModelType::Custom {
        model_id: "cross-encoder/ms-marco-MiniLM-L-12-v2".to_string(),
        max_length: Some(512),
    },
    batch_size: 16,
    show_download_progress: true,
    ..Default::default()
};
```

```rust title="Rust"
use xberg::{LlmConfig, RerankerConfig, RerankerModelType};

let config = RerankerConfig {
    model: RerankerModelType::Llm {
        llm: LlmConfig {
            model: "cohere/rerank-english-v3.0".to_string(),
            api_key: Some(std::env::var("COHERE_API_KEY").unwrap_or_default()),
            ..Default::default()
        },
    },
    top_k: Some(5),
    max_rerank_duration_secs: Some(30),
    ..Default::default()
};
```

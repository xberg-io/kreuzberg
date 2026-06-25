```rust title="Rust"
use async_trait::async_trait;
use xberg::plugins::{Plugin, RerankerBackend, register_reranker_backend};
use xberg::{rerank, XbergError, RerankerConfig, RerankerModelType, Result};
use std::sync::Arc;

struct MyReranker;

impl Plugin for MyReranker {
    fn name(&self) -> &str { "my-reranker" }
    fn version(&self) -> String { "1.0.0".to_string() }
    fn initialize(&self) -> Result<()> { Ok(()) }
    fn shutdown(&self) -> Result<()> { Ok(()) }
}

#[async_trait]
impl RerankerBackend for MyReranker {
    async fn rerank(&self, _query: String, documents: Vec<String>) -> Result<Vec<f32>> {
        // Return raw scores in input order; dispatcher sorts and truncates.
        Ok((0..documents.len()).map(|i| 0.5 + i as f32 * 0.1).collect())
    }
}

register_reranker_backend(Arc::new(MyReranker))?;

let config = RerankerConfig {
    model: RerankerModelType::Plugin { name: "my-reranker".to_string() },
    ..Default::default()
};
let _ = rerank("query".to_string(), vec!["doc".to_string()], &config)?;
# Ok::<(), XbergError>(())
```

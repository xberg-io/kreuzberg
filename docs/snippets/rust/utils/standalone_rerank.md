```rust title="Rust"
use xberg::{rerank, RerankerConfig, RerankerModelType};

let query = "How to train a dog".to_string();
let documents = vec![
    "Dog training requires patience and consistency.".to_string(),
    "Cats are independent animals that prefer to play alone.".to_string(),
    "Bird care includes proper cage setup and regular cleaning.".to_string(),
];

let config = RerankerConfig {
    model: RerankerModelType::Preset { name: "fast".to_string() },
    top_k: Some(2),
    ..Default::default()
};

let results = rerank(query, documents, &config)?;
for r in results {
    println!("#{}: {:.3} — {}", r.index, r.score, r.document);
}
# Ok::<(), xberg::XbergError>(())
```

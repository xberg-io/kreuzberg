```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, ChunkingConfig};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 1000,
            overlap: 200,
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;
    println!("Chunks: {}", output.results[0].chunks.len());
    for chunk in &output.results[0].chunks {
        println!("Length: {}", chunk.content.len());
    }
    Ok(())
}
```

```rust title="Rust - Markdown with Heading Context"
use xberg::{extract, ExtractionConfig, ExtractInput, ChunkingConfig, ChunkerType, ChunkSizing};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 500,
            overlap: 50,
            chunker_type: ChunkerType::Markdown,
            sizing: ChunkSizing::Tokenizer {
                model: "Xenova/gpt-4o".into(),
                cache_dir: None,
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.md"), &config).await?;
    for chunk in &output.results[0].chunks {
        if let Some(heading_context) = &chunk.metadata.heading_context {
            for heading in &heading_context.headings {
                println!("Heading L{}: {}", heading.level, heading.text);
            }
        }
        println!("Content: {}...", &chunk.content[..100.min(chunk.content.len())]);
    }
    Ok(())
}
```

```rust title="Rust - Prepend Heading Context"
use xberg::{extract, ExtractionConfig, ExtractInput, ChunkingConfig, ChunkerType};

#[tokio::main]
async fn main() -> xberg::Result<()> {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: 500,
            overlap: 50,
            chunker_type: ChunkerType::Markdown,
            prepend_heading_context: true,
            ..Default::default()
        }),
        ..Default::default()
    };

    let output = extract(ExtractInput::from_uri("document.md"), &config).await?;
    for chunk in &output.results[0].chunks {
        // Each chunk's content is prefixed with its heading breadcrumb
        println!("Content: {}...", &chunk.content[..100.min(chunk.content.len())]);
    }
    Ok(())
}
```

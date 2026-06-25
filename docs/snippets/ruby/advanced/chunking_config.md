```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  chunking: Xberg::ChunkingConfig.new(
    max_characters: 1000,
    overlap: 200,
    embedding: Xberg::EmbeddingConfig.new(
      model: Xberg::EmbeddingModelType.new(
        type: 'preset',
        name: 'all-minilm-l6-v2'
      ),
      normalize: true,
      batch_size: 32
    )
  )
)
```

```ruby title="Ruby - Prepend Heading Context"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  chunking: Xberg::ChunkingConfig.new(
    chunker_type: "markdown",
    max_characters: 500,
    overlap: 50,
    prepend_heading_context: true
  )
)
```

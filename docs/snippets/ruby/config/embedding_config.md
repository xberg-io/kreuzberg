```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  chunking: Xberg::ChunkingConfig.new(
    max_characters: 1000,
    embedding: Xberg::EmbeddingConfig.new(
      model: Xberg::EmbeddingModelType.new(
        type: 'preset',
        name: 'all-mpnet-base-v2'
      ),
      batch_size: 16,
      normalize: true,
      show_download_progress: true
    )
  )
)
```

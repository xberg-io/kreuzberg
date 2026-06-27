```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  chunking: Xberg::ChunkingConfig.new(
    max_characters: 500,
    overlap: 50,
    embedding: Xberg::EmbeddingConfig.new(
      model: Xberg::EmbeddingModelType.new(
        type: 'preset',
        name: 'balanced'
      ),
      normalize: true
    )
  )
)

input = Xberg::ExtractInput.new(uri: 'research_paper.pdf')
result = Xberg.extract(input, config)
chunks = result.results.first.chunks

chunks.each_with_index do |chunk, i|
  puts "Chunk #{i + 1}/#{chunks.length}"
  puts "Position: #{chunk.metadata[:byte_start]}-#{chunk.metadata[:byte_end]}"
  puts "Content: #{chunk.content[0..99]}..."
  puts "Embedding: #{chunk.embedding.length} dimensions" if chunk.embedding
end
```

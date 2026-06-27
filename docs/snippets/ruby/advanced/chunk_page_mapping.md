```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  chunking: Xberg::ChunkingConfig.new(max_characters: 500, overlap: 50),
)

input = Xberg::ExtractInput.new(uri: 'document.pdf')
result = Xberg.extract(input, config)

result.results.first.chunks&.each do |chunk|
  first = chunk.metadata.first_page
  last = chunk.metadata.last_page
  next if first.nil?

  page_range = first == last ? "Page #{first}" : "Pages #{first}-#{last}"
  puts "Chunk: #{chunk.content[0..50]}... (#{page_range})"
end
```

```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  token_reduction: Xberg::TokenReductionConfig.new(
    mode: 'moderate',
    preserve_markdown: true
  )
)

input = Xberg::ExtractInput.new(uri: 'verbose_document.pdf')
result = Xberg.extract(input, config)

# Check reduction statistics in metadata
original_tokens = result.results.first.metadata['original_token_count']
reduced_tokens = result.results.first.metadata['token_count']
reduction_ratio = result.results.first.metadata['token_reduction_ratio']

puts "Reduced from #{original_tokens} to #{reduced_tokens} tokens"
puts "Reduction: #{reduction_ratio * 100}%"
```

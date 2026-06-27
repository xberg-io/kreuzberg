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
first_result = result.results.first

original_tokens = first_result.metadata&.dig('original_token_count') || 0
reduced_tokens = first_result.metadata&.dig('token_count') || 0
reduction_ratio = first_result.metadata&.dig('token_reduction_ratio') || 0.0

puts "Reduced from #{original_tokens} to #{reduced_tokens} tokens"
puts "Reduction: #{(reduction_ratio * 100).round(1)}%"
```

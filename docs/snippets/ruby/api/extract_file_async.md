```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  use_cache: false,
  enable_quality_processing: true
)

result = Xberg.extract_file_async('document.pdf', config: config)

puts "Async extraction complete"
puts "Extracted #{result.content.length} characters"
puts "Quality: #{result.quality_score}"
```

```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  use_cache: true,
  enable_quality_processing: true
)

result = Xberg.extract_file_sync('document.pdf', config: config)

puts "Extracted #{result.content.length} characters"
puts "MIME type: #{result.mime_type}"
puts "Quality score: #{result.quality_score}"
```

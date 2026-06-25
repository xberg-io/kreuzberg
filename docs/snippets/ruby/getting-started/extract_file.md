```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  use_cache: true,
  enable_quality_processing: true
)

result = Xberg.extract_file_sync('contract.pdf', config: config)

puts "Extracted #{result.content.length} characters"
puts "Quality score: #{result.quality_score}"
puts "Processing time: #{result.metadata&.dig('processing_time')}ms"
```

```ruby title="Ruby"
require 'xberg'

pdf_bytes = File.read('document.pdf')
config = Xberg::ExtractionConfig.new(
  enable_quality_processing: true
)

result = Xberg.extract_bytes_async(
  pdf_bytes,
  'application/pdf',
  config: config
)

puts "Async bytes extraction done"
puts "Content preview: #{result.content[0..100]}"
puts "Quality score: #{result.quality_score}"
```

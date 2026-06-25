```ruby title="Ruby"
require 'xberg'

pdf_bytes = File.read('document.pdf')
config = Xberg::ExtractionConfig.new(
  use_cache: true
)

result = Xberg.extract_bytes_sync(
  pdf_bytes,
  'application/pdf',
  config: config
)

puts "Extracted #{result.content.length} characters"
puts "Detected MIME: #{result.mime_type}"
```

```ruby title="Ruby"
require 'xberg'

# Register custom extractor with priority 50
Xberg.register_document_extractor(
  name: "custom-json-extractor",
  extractor: ->(content, mime_type, config) {
    JSON.parse(content.to_s)
  },
  priority: 50
)

input = Xberg::ExtractInput.new(uri: "document.json")
config = Xberg::ExtractionConfig.new
result = Xberg.extract(input, config)
puts "Extracted content length: #{result.results.first.content.length}"
```

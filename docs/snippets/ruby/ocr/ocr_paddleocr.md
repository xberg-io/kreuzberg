```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  ocr: Xberg::OcrConfig.new(
    backend: 'paddleocr',
    language: 'eng'
    # model_tier: 'server' # for max accuracy
  )
)

input = Xberg::ExtractInput.new(uri: 'scanned.pdf')
result = Xberg.extract(input, config)
puts result.results.first.content[0..100]
puts "Total length: #{result.results.first.content.length}"
```

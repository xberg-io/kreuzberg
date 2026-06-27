```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  ocr: Xberg::OcrConfig.new(
    backend: 'tesseract',
    language: 'eng+deu+fra'
  )
)

input = Xberg::ExtractInput.new(uri: 'multilingual.pdf')
result = Xberg.extract(input, config)
puts result.results.first.content
```

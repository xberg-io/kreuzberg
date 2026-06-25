```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  ocr: Xberg::OcrConfig.new(
    backend: 'tesseract',
    language: 'eng+fra',
    tesseract_config: Xberg::TesseractConfig.new(psm: 3)
  )
)
```

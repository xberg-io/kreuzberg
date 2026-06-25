```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  ocr: Xberg::OcrConfig.new(
    language: 'eng+fra+deu',
    tesseract_config: Xberg::TesseractConfig.new(
      psm: 6,
      oem: 1,
      min_confidence: 0.8,
      tessedit_char_whitelist: 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 .,!?',
      enable_table_detection: true
    )
  )
)
```

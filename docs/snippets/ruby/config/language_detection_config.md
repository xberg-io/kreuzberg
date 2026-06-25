```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  language_detection: Xberg::LanguageDetectionConfig.new(
    enabled: true,
    min_confidence: 0.8,
    detect_multiple: false
  )
)
```

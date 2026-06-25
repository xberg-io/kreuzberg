```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  postprocessor: Xberg::PostProcessorConfig.new(
    enabled: true,
    enabled_processors: ['deduplication', 'whitespace_normalization'],
    disabled_processors: ['mojibake_fix']
  )
)
```

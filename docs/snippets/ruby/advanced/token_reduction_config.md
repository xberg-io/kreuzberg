```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  token_reduction: Xberg::TokenReductionConfig.new(
    mode: 'moderate',
    preserve_markdown: true,
    preserve_code: true,
    language_hint: 'eng'
  )
)
```

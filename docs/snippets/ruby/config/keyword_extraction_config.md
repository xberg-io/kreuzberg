```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  keywords: Xberg::KeywordConfig.new(
    algorithm: Xberg::KeywordAlgorithm::YAKE,
    max_keywords: 10,
    min_score: 0.3,
    ngram_range: [1, 3],
    language: 'en'
  )
)
```

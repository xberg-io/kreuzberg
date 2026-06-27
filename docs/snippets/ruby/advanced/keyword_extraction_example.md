```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  keywords: Xberg::KeywordConfig.new(
    algorithm: Xberg::KeywordAlgorithm::YAKE,
    max_keywords: 10,
    min_score: 0.3
  )
)

input = Xberg::ExtractInput.new(uri: 'research_paper.pdf')
output = Xberg.extract(input, config)
result = output.results.first

keywords = result.extracted_keywords || []
keywords.each do |kw|
  text = kw.text
  score = kw.score
  puts "#{text}: #{score.round(3)}"
end
```

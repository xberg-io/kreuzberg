```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  enable_quality_processing: true
)

input = Xberg::ExtractInput.new(uri: 'scanned_document.pdf')
result = Xberg.extract(input, config)
first_result = result.results.first

quality_score = first_result.quality_score || 0.0

if quality_score < 0.5
  puts "Warning: Low quality extraction (#{quality_score.round(2)})"
  puts "Consider re-scanning with higher DPI or adjusting OCR settings"
else
  puts "Quality score: #{quality_score.round(2)}"
end
```

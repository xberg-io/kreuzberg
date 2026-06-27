```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  language_detection: Xberg::LanguageDetectionConfig.new(
    enabled: true,
    min_confidence: 0.8,
    detect_multiple: false
  )
)

input = Xberg::ExtractInput.new(uri: 'document.pdf')
result = Xberg.extract(input, config)
first_result = result.results.first

if first_result.detected_languages&.any?
  puts "Detected Language: #{first_result.detected_languages.first}"
else
  puts "No language detected"
end

puts "Content length: #{first_result.content.length} characters"
```

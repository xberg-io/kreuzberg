```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  language_detection: Xberg::LanguageDetectionConfig.new(
    enabled: true,
    min_confidence: 0.8,
    detect_multiple: true
  )
)

input = Xberg::ExtractInput.new(uri: 'multilingual_document.pdf')
result = Xberg.extract(input, config)
first_result = result.results.first

languages = first_result.detected_languages || []

if languages.any?
  puts "Detected #{languages.length} language(s): #{languages.join(', ')}"
else
  puts "No languages detected"
end

puts "Total content: #{first_result.content.length} characters"
puts "MIME type: #{first_result.mime_type}"
```

```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  ocr: Xberg::OcrConfig.new(
    backend: 'paddleocr',
    language: 'eng'
  )
)

input = Xberg::ExtractInput.new(uri: 'scanned.pdf')
result = Xberg.extract(input, config)

result.results.first.ocr_elements&.each do |element|
  puts "Text: #{element.text}"
  puts "Confidence: #{format('%.2f', element.confidence.recognition)}"
  puts "Geometry: #{element.geometry}"
  if element.rotation
    puts "Rotation: #{element.rotation.angle}°"
  end
  puts
end
```

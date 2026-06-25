```ruby title="Ruby"
require 'xberg'

items = [
  Xberg::BatchFileItem.new(path: 'doc1.pdf'),
  Xberg::BatchFileItem.new(path: 'doc2.docx'),
  Xberg::BatchFileItem.new(path: 'doc3.pptx')
]

config = Xberg::ExtractionConfig.new(use_cache: true)

results = Xberg.batch_extract_files_sync(items, config: config)

results.each_with_index do |result, idx|
  puts "Document #{idx + 1}:"
  puts "  Extracted: #{result.content.length} characters"
  puts "  Quality: #{result.quality_score}"
  puts "  MIME: #{result.mime_type}"
end
```

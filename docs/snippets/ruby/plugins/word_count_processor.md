```ruby title="Ruby"
require 'xberg'

class WordCountProcessor
  def call(result)
    return result if result['content'].empty?
    word_count = result['content'].split.length
    result['metadata'] ||= {}
    result['metadata']['word_count'] = word_count
    result
  end
end

processor = WordCountProcessor.new
Xberg.register_post_processor('word_count', processor, 10)

config = Xberg::ExtractionConfig.new(
  postprocessor: { enabled: true }
)

input = Xberg::ExtractInput.new(uri: 'document.pdf')
result = Xberg.extract(input, config)
puts "Words: #{result.results.first.metadata['word_count']}"
```

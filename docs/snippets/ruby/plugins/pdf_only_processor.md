```ruby title="Ruby"
require 'xberg'

class PdfOnlyProcessor
  def call(result)
    return result unless result['mime_type'] == 'application/pdf'
    result['metadata'] ||= {}
    result['metadata']['pdf_processed'] = true
    result
  end
end

processor = PdfOnlyProcessor.new
Xberg.register_post_processor('pdf_only', processor)

config = Xberg::ExtractionConfig.new(
  postprocessor: { enabled: true }
)

input = Xberg::ExtractInput.new(uri: 'document.pdf')
result = Xberg.extract(input, config)
puts "Metadata: #{result.results.first.metadata.inspect}"
```

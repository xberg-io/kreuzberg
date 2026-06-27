```ruby title="Ruby"
require 'xberg'
require 'json'

schema = {
  type: 'object',
  properties: {
    title: { type: 'string' },
    authors: { type: 'array', items: { type: 'string' } },
    date: { type: 'string' }
  },
  required: %w[title authors date],
  additionalProperties: false
}

config = Xberg::ExtractionConfig.new(
  structured_extraction: Xberg::StructuredExtractionConfig.new(
    schema: JSON.generate(schema),
    schema_name: 'PaperMetadata',
    strict: true,
    llm: Xberg::LlmConfig.new(model: 'openai/gpt-4o-mini')
  )
)

input = Xberg::ExtractInput.new(uri: 'paper.pdf')
result = Xberg.extract(input, config)
puts result.results.first.structured_output
```

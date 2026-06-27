```ruby title="Ruby"
require "xberg"

validator = lambda do |result|
  raise StandardError, "Content too short" if result.content.length < 50
end

Xberg.register_validator("min_length", validator, priority: 10)

input = Xberg::ExtractInput.new(uri: "document.pdf")
config = Xberg::ExtractionConfig.new
result = Xberg.extract(input, config)
puts "Validated content length: #{result.results.first.content.length}"

Xberg.unregister_validator("min_length")
```

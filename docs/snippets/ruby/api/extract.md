```ruby title="Ruby"
require "xberg"

output = Xberg.extract(Xberg::ExtractInput.new(uri: "document.pdf"))

puts output.results.first.content
puts "Results: #{output.summary.results}"
```

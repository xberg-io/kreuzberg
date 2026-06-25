```ruby title="Ruby"
require 'xberg'

processors = Xberg.list_post_processors
validators = Xberg.list_validators
backends = Xberg.list_ocr_backends

puts "Post-processors: #{processors.inspect}"
puts "Validators: #{validators.inspect}"
puts "OCR backends: #{backends.inspect}"
```

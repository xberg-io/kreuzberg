```ruby title="Ruby"
require 'xberg'

class CustomPostProcessor
  def call(result)
    result['metadata'] ||= {}
    result['metadata']['processed_by'] = 'CustomPostProcessor'
    result
  end
end

class CustomValidator
  def call(result)
    raise StandardError, 'Empty' if result['content'].empty?
  end
end

processor = CustomPostProcessor.new
validator = CustomValidator.new

Xberg.register_post_processor('custom', processor)
Xberg.register_validator('custom', validator)
```

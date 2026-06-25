```ruby title="Ruby"
require 'xberg'
require 'rspec'

describe 'Plugin Testing' do
  it 'registers and calls post-processor' do
    processor = ->(result) { result['metadata'] ||= {}; result }
    Xberg.register_post_processor('test', processor)
    expect(Xberg.list_post_processors).to include('test')
    Xberg.unregister_post_processor('test')
  end

  it 'registers and validates' do
    validator = ->(result) do
      raise StandardError, 'Too short' if result['content'].length < 10
    end
    Xberg.register_validator('test-val', validator)
    expect(Xberg.list_validators).to include('test-val')
    Xberg.unregister_validator('test-val')
  end
end
```

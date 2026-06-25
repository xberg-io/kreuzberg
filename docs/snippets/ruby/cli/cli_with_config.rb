```ruby title="cli_with_config.rb"
require 'json'
require 'open3'

def extract_with_config(file_path, config_path)
  stdout, stderr, status = Open3.capture3(
    'xberg', 'extract', file_path, '--config', config_path, '--format', 'json'
  )

  unless status.success?
    warn "Error: #{stderr}"
    exit 1
  end

  JSON.parse(stdout)
end

config_file = 'xberg.toml'
document = 'document.pdf'

puts "Extracting #{document} with config #{config_file}"
result = extract_with_config(document, config_file)

puts "Content length: #{result['content'].length}"
puts "Format: #{result['format']}"
puts "Languages: #{result['languages'].join(', ')}"
```

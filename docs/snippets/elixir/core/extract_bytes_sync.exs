```elixir title="Elixir"
# Read file into memory
{:ok, file_content} = File.read("document.pdf")

# Extract from bytes/binary data
{:ok, result} = Xberg.extract(file_content, "application/pdf")

content = result.content
IO.puts("Extracted content:")
IO.puts(content)
IO.puts("MIME type: #{result.mime_type}")
IO.puts("Tables found: #{length(result.tables)}")
```

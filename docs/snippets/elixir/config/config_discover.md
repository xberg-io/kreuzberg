```elixir title="Elixir"
# Extract with nil config to use discovered/default configuration
{:ok, output} = Xberg.extract(input: %Xberg.ExtractInput{kind: :uri, uri: "document.pdf", mime_type: "application/pdf"}, config: nil)
result = List.first(output.results)
IO.puts(result.content)
```

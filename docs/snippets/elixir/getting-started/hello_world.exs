```elixir title="Elixir"
# First Xberg program - extract text from a PDF
{:ok, output} = Xberg.extract(input: %Xberg.ExtractInput{kind: :uri, uri: "document.pdf"}, config: nil)
result = List.first(output.results)
IO.puts(result.content)
```

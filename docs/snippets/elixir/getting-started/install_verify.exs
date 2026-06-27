```elixir title="Elixir"
# Verify Xberg is installed and working by extracting a document
{:ok, output} = Xberg.extract(input: %Xberg.ExtractInput{kind: :uri, uri: "sample.pdf"}, config: nil)
result = List.first(output.results)
IO.puts("Installation verified! Extracted #{String.length(result.content)} characters")
```

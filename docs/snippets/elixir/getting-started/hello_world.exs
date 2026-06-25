```elixir title="Elixir"
# First Xberg program - extract text from a PDF
{:ok, result} = Xberg.extract_file("document.pdf")
IO.puts(result.content)
```

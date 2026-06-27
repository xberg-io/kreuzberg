```elixir title="Elixir"
input = %Xberg.ExtractInput{kind: :uri, uri: "document.pdf"}

{:ok, output} = Xberg.extract(input: input, config: nil)

IO.inspect(output.summary)
```

```elixir title="Elixir"
# Example: Handling extraction errors
case Xberg.extract(input: %Xberg.ExtractInput{kind: :uri, uri: "document.pdf"}, config: nil) do
  {:ok, output} ->
    result = List.first(output.results)
    IO.puts("Successfully extracted content")
    IO.puts("Content length: #{byte_size(result.content)} characters")

  {:error, reason} ->
    IO.puts("Extraction failed: #{reason}")
end

# Example: Handling with custom error message
result = Xberg.extract(input: %Xberg.ExtractInput{kind: :uri, uri: "nonexistent.pdf"}, config: nil)

case result do
  {:ok, output} ->
    data = List.first(output.results)
    IO.puts("File processed successfully")
  {:error, error} ->
    IO.puts("Error details: #{inspect(error)}")
end

# Example: Extract with pattern matching
case Xberg.extract(input: %Xberg.ExtractInput{kind: :bytes, bytes: <<>>, mime_type: "application/pdf"}, config: nil) do
  {:ok, output} ->
    result = List.first(output.results)
    IO.puts("Content: #{result.content}")
  {:error, msg} when is_binary(msg) ->
    IO.puts("Validation error: #{msg}")
  {:error, reason} ->
    IO.puts("Unknown error: #{inspect(reason)}")
end
```

```elixir title="Document Structure Config (Elixir)"
config = %Xberg.ExtractionConfig{
  include_document_structure: true
}

{:ok, result} = Xberg.extract_file_sync("document.pdf", config)

if result.document do
  Enum.each(result.document.nodes, fn node ->
    IO.puts("[#{node.content.node_type}]")
  end)
end
```

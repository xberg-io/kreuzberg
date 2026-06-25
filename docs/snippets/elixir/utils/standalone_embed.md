```elixir
# Embed with default config
{:ok, embeddings} = Xberg.embed(["Hello world", "How are you?"])

# Embed with specific preset
config = %Xberg.EmbeddingConfig{model: {:preset, "fast"}}
{:ok, embeddings} = Xberg.embed(["Hello world"], config)

# Raise on error
embeddings = Xberg.embed!(["Hello world"])
```

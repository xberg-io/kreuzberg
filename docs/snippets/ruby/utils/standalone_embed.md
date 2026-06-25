```ruby title="Ruby"
require "xberg"

config = { model: { type: "preset", name: "balanced" }, normalize: true }
texts = ["Hello, world!", "Xberg is fast"]

# Synchronous
embeddings = Xberg.embed_sync(texts: texts, config: config)
puts embeddings.length    # 2
puts embeddings[0].length # 768

# Async variant (uses same thread, returns when done)
embeddings = Xberg.embed(texts: texts, config: config)
puts embeddings[0].length # 768
```

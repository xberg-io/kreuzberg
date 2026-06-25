```python title="Python"
from xberg import embed_sync, embed, EmbeddingConfig, EmbeddingModelType

# Synchronous
embeddings = embed_sync(
    ["Hello, world!", "Xberg is fast"],
    config=EmbeddingConfig(model=EmbeddingModelType.preset("balanced"), normalize=True),
)
assert len(embeddings) == 2
assert len(embeddings[0]) == 768

# Asynchronous
async def main():
    embeddings = await embed(
        ["Hello, world!", "Xberg is fast"],
        config=EmbeddingConfig(model=EmbeddingModelType.preset("balanced"), normalize=True),
    )
    assert len(embeddings) == 2
```

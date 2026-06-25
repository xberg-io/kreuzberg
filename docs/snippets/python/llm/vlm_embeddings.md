```python title="Python"
import asyncio
from xberg import embed, EmbeddingConfig, EmbeddingModelType, LlmConfig

async def main() -> None:
    config = EmbeddingConfig(
        model=EmbeddingModelType.llm(
            LlmConfig(model="openai/text-embedding-3-small")
        ),
        normalize=True,
    )
    embeddings = await embed(["Hello world"], config=config)
    print(len(embeddings[0]))  # 1536

asyncio.run(main())
```

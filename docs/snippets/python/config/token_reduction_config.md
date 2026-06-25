```python title="Python"
from xberg import ExtractionConfig, TokenReductionConfig

config: ExtractionConfig = ExtractionConfig(
    token_reduction=TokenReductionConfig(
        mode="moderate",
        preserve_important_words=True,
    )
)
```

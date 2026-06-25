```python title="Python"
import asyncio
from xberg import extract_file, ExtractionConfig, OcrConfig, LlmConfig

async def main() -> None:
    config = ExtractionConfig(
        force_ocr=True,
        ocr=OcrConfig(
            backend="vlm",
            vlm_config=LlmConfig(model="openai/gpt-4o-mini"),
        ),
    )
    result = await extract_file("scan.pdf", config=config)
    print(result.content)

asyncio.run(main())
```

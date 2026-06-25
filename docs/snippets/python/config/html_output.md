```python title="Python"
import asyncio
from xberg import ExtractionConfig, extract_file

async def main() -> None:
    config = ExtractionConfig(
        output_format="html",
        html_output={
            "theme": "github",
            "embed_css": True,
        },
    )
    result = await extract_file("document.pdf", config=config)
    print(result.content)  # HTML with kb-* classes and GitHub theme

asyncio.run(main())
```

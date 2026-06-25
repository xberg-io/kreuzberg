```python title="Python"
import asyncio
from xberg import ExtractionConfig, PdfConfig, HierarchyConfig, extract_file

async def main() -> None:
    config: ExtractionConfig = ExtractionConfig(
        pdf_options=PdfConfig(
            extract_images=True,
            extract_metadata=True,
            passwords=["password1", "password2"],
            hierarchy=HierarchyConfig(enabled=True, k_clusters=6)
        )
    )
    result = await extract_file("document.pdf", config=config)
    print(f"Content: {result.content[:100]}")

asyncio.run(main())
```

```python title="Python"
import asyncio
from xberg import extract_file, __version__

async def main() -> None:
    print(f"Xberg version: {__version__}")

    result = await extract_file("document.pdf")
    print(f"Extraction successful: {len(result.content) > 0}")

asyncio.run(main())
```

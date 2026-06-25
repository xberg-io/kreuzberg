```python title="Python"
from xberg import extract_file_sync, ExtractionConfig, XbergError

config = ExtractionConfig()

try:
    result = extract_file_sync("missing.pdf", config=config)
except XbergError as e:
    print(f"Extraction failed: {e}")
    raise
```

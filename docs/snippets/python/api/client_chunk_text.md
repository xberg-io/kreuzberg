```python title="Python"
import httpx

# Basic chunking with defaults
with httpx.Client() as client:
    response = client.post(
        "http://localhost:8000/chunk",
        json={"text": "Your long text content here..."}
    )
    result = response.json()
    for chunk in result["chunks"]:
        print(f"Chunk {chunk['chunk_index']}: {chunk['content'][:50]}...")

# Chunking with custom configuration
with httpx.Client() as client:
    response = client.post(
        "http://localhost:8000/chunk",
        json={
            "text": "Your long text content here...",
            "chunker_type": "text",
            "config": {
                "max_characters": 1000,
                "overlap": 50,
                "trim": True
            }
        }
    )
    result = response.json()
    print(f"Created {result['chunk_count']} chunks")
```

"""Basic Extraction Example.

Demonstrates basic document extraction with Kreuzberg.
"""

from kreuzberg import ExtractionConfig, extract_file, extract_file_sync


def main() -> None:
    # Simple synchronous extraction
    result = extract_file_sync("document.pdf")
    print(f"Extracted {len(result.content)} characters")
    print(f"Content preview: {result.content[:200]}...")

    # Extraction with configuration
    config = ExtractionConfig(
        enable_quality_processing=True,
        use_cache=True,
    )
    result = extract_file_sync("document.pdf", config=config)
    print(f"With quality processing: {len(result.content)} characters")

    # Async extraction
    import asyncio

    async def async_extract():
        return await extract_file("document.pdf")

    result = asyncio.run(async_extract())
    print(f"Async result: {len(result.content)} characters")

    # Extraction from bytes
    from kreuzberg import extract_bytes_sync

    with open("document.pdf", "rb") as f:
        data = f.read()

    result = extract_bytes_sync(data, mime_type="application/pdf")
    print(f"From bytes: {len(result.content)} characters")

    # Accessing typed metadata
    result = extract_file_sync("document.pdf")
    if result.metadata.pdf:
        print(f"Page count: {result.metadata.pdf.page_count}")
        print(f"Title: {result.metadata.title}")


if __name__ == "__main__":
    main()

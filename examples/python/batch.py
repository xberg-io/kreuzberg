"""Batch Processing Example.

Demonstrates efficient batch processing of multiple documents.
"""

import asyncio
from pathlib import Path

from kreuzberg import ExtractionConfig, batch_extract_files, batch_extract_files_sync


def main() -> None:
    # Basic batch extraction
    files = [
        "document1.pdf",
        "document2.docx",
        "document3.txt",
        "document4.html",
    ]

    results = batch_extract_files_sync(files)

    for file, result in zip(files, results, strict=False):
        print(f"{file}: {len(result.content)} chars, format={result.metadata.mime_type}")

    # Async batch extraction
    async def process_batch():
        files = [f"doc{i}.pdf" for i in range(10)]
        results = await batch_extract_files(files)

        total_chars = sum(len(r.content) for r in results)
        print(f"Batch total: {total_chars} characters across {len(results)} files")

        return results

    asyncio.run(process_batch())

    # Batch with configuration
    config = ExtractionConfig(
        enable_quality_processing=True,
        use_cache=True,
        ocr=None,
    )

    results = batch_extract_files_sync(files, config=config)
    print(f"Configured batch: {len(results)} results")

    # Batch from glob pattern
    from glob import glob

    pdf_files = glob("data/*.pdf")
    if pdf_files:
        results = batch_extract_files_sync(pdf_files[:5])

        for file, result in zip(pdf_files[:5], results, strict=False):
            print(f"{Path(file).name}: {len(result.content)} chars")

    # Batch extraction from bytes
    from kreuzberg import batch_extract_bytes_sync

    data_list = []
    mime_types = []

    for file in files[:3]:
        with open(file, "rb") as f:
            data_list.append(f.read())

        ext = Path(file).suffix.lower()
        mime_map = {
            ".pdf": "application/pdf",
            ".docx": "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ".txt": "text/plain",
            ".html": "text/html",
        }
        mime_types.append(mime_map.get(ext, "application/octet-stream"))

    results = batch_extract_bytes_sync(data_list, mime_types)
    print(f"Bytes batch: {len(results)} results")

    # Error handling with individual file processing
    files_with_invalid = [
        "valid1.pdf",
        "nonexistent.pdf",
        "valid2.txt",
    ]

    for file in files_with_invalid:
        try:
            result = batch_extract_files_sync([file])[0]
            print(f"{file}: OK ({len(result.content)} chars)")
        except Exception as e:
            print(f"{file}: FAILED ({type(e).__name__}: {e})")


if __name__ == "__main__":
    main()

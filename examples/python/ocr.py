"""OCR Extraction Example.

Demonstrates OCR extraction from scanned PDFs and images.
"""

from kreuzberg import ExtractionConfig, OcrConfig, extract_file_sync


def main() -> None:
    # Basic OCR with Tesseract
    config = ExtractionConfig(
        ocr=OcrConfig(
            backend="tesseract",
            language="eng",
        )
    )

    result = extract_file_sync("scanned_document.pdf", config=config)
    print(f"OCR extracted: {len(result.content)} characters")
    print(f"Preview: {result.content[:200]}...")

    # OCR with a different language (German)
    config = ExtractionConfig(
        ocr=OcrConfig(
            backend="tesseract",
            language="deu",
        )
    )

    result = extract_file_sync("german_document.pdf", config=config)
    print(f"German OCR: {len(result.content)} characters")

    # Force OCR on a document with mixed native text and scanned pages
    config = ExtractionConfig(
        ocr=OcrConfig(backend="tesseract", language="eng"),
        force_ocr=True,
    )

    result = extract_file_sync("mixed_document.pdf", config=config)
    print(f"Forced OCR: {len(result.content)} characters")

    # OCR on a screenshot/image file
    config = ExtractionConfig(ocr=OcrConfig(backend="tesseract", language="eng"))

    result = extract_file_sync("screenshot.png", config=config)

    if result.metadata.ocr:
        print(f"OCR confidence: {result.metadata.ocr.confidence}")
        print(f"OCR backend used: {result.metadata.ocr.backend}")

    # OCR with table detection via Tesseract
    from kreuzberg import TesseractConfig

    config = ExtractionConfig(
        ocr=OcrConfig(
            backend="tesseract",
            language="eng",
            tesseract_config=TesseractConfig(
                enable_table_detection=True,
            ),
        )
    )

    result = extract_file_sync("table_document.pdf", config=config)

    for i, table in enumerate(result.tables):
        print(f"Table {i + 1}: {len(table.rows)} rows x {len(table.headers)} columns")
        print(f"  Headers: {table.headers}")


if __name__ == "__main__":
    main()

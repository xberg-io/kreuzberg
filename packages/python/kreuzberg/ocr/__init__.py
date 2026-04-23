"""Python OCR backend implementations.

Register backends explicitly:

    from kreuzberg import register_ocr_backend
    from kreuzberg.ocr import EasyOCRBackend

    register_ocr_backend(EasyOCRBackend(use_gpu=True, languages=["en"]))

Install optional backends:
    pip install "kreuzberg[easyocr]"
"""

from __future__ import annotations

__all__ = ["EasyOCRBackend", "OcrBackendProtocol"]

from kreuzberg.ocr.protocol import OcrBackendProtocol

try:
    from kreuzberg.ocr.easyocr import EasyOCRBackend
except ImportError:
    EasyOCRBackend = None  # type: ignore[assignment,misc]

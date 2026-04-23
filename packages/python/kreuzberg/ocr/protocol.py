"""Protocol for Python OCR backends compatible with Rust FFI bridge."""

from __future__ import annotations

from typing import Any, Protocol


class OcrBackendProtocol(Protocol):
    """Protocol for OCR backends registered with the Rust extraction core.

    Required Methods:
        name: Return backend name (e.g., 'easyocr')
        supported_languages: Return list of supported language codes
        process_image: Process image bytes and return extraction result

    Optional Methods:
        initialize, shutdown, version, process_document, supports_document_processing
    """

    def name(self) -> str: ...
    def supported_languages(self) -> list[str]: ...
    def process_image(self, image_bytes: bytes, language: str) -> dict[str, Any]: ...
    def process_image_file(self, path: str, language: str) -> dict[str, Any]: ...
    def supports_document_processing(self) -> bool: ...
    def process_document(self, path: str, language: str) -> dict[str, Any]: ...
    def initialize(self) -> None: ...
    def shutdown(self) -> None: ...

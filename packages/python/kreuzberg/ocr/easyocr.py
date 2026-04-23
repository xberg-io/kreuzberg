"""EasyOCR backend for document OCR processing.

Install: pip install "kreuzberg[easyocr]"
"""

from __future__ import annotations

import logging
from typing import Any

from kreuzberg.exceptions import OcrError, ValidationError

logger = logging.getLogger(__name__)

SUPPORTED_LANGUAGES = {
    "abq", "ady", "af", "ang", "ar", "as", "ava", "az",
    "be", "bg", "bh", "bho", "bn", "bs",
    "ch_sim", "ch_tra", "che", "cs", "cy",
    "da", "dar", "de", "en", "es", "et",
    "fa", "fr", "ga", "gom",
    "hi", "hr", "hu", "id", "inh", "is", "it", "ja",
    "kbd", "kn", "ko", "ku",
    "la", "lbe", "lez", "lt", "lv",
    "mah", "mai", "mi", "mn", "mr", "ms", "mt",
    "ne", "new", "nl", "no", "oc",
    "pi", "pl", "pt", "ro", "ru", "rs_cyrillic", "rs_latin",
    "sck", "sk", "sl", "sq", "sv", "sw",
    "ta", "tab", "te", "th", "tjk", "tl", "tr",
    "ug", "uk", "ur", "uz", "vi",
}


class EasyOCRBackend:
    """EasyOCR backend supporting 80+ languages with optional GPU acceleration.

    Args:
        languages: Language codes to enable (default: ["en"]).
        use_gpu: Force GPU usage. None = auto-detect CUDA.
        model_storage_directory: EasyOCR model cache directory.
        beam_width: Beam width for recognition.
    """

    def __init__(
        self,
        *,
        languages: list[str] | None = None,
        use_gpu: bool | None = None,
        model_storage_directory: str | None = None,
        beam_width: int = 5,
    ) -> None:
        try:
            import easyocr as easyocr_module  # noqa: PLC0415
        except ImportError as e:
            msg = 'EasyOCR requires the "easyocr" package. Install: pip install "kreuzberg[easyocr]"'
            raise ImportError(msg) from e

        self._easyocr_module = easyocr_module
        self.languages = languages or ["en"]
        self.beam_width = beam_width
        self.model_storage_directory = model_storage_directory

        unsupported = [lang for lang in self.languages if lang not in SUPPORTED_LANGUAGES]
        if unsupported:
            msg = f"Unsupported EasyOCR languages: {', '.join(unsupported)}"
            raise ValidationError(msg)

        self.use_gpu = self._is_cuda_available() if use_gpu is None else use_gpu
        self._reader: Any | None = None

    def name(self) -> str:
        return "easyocr"

    def supported_languages(self) -> list[str]:
        return sorted(SUPPORTED_LANGUAGES)

    def initialize(self) -> None:
        if self._reader is not None:
            return
        try:
            self._reader = self._easyocr_module.Reader(
                self.languages, gpu=self.use_gpu, verbose=False,
                model_storage_directory=self.model_storage_directory,
            )
        except Exception as e:
            raise OcrError(f"Failed to initialize EasyOCR: {e}") from e

    def shutdown(self) -> None:
        self._reader = None

    def supports_document_processing(self) -> bool:
        return False

    def process_image(self, image_bytes: bytes, language: str) -> dict[str, Any]:
        if self._reader is None:
            self.initialize()
        if self._reader is None:
            raise RuntimeError("EasyOCR reader failed to initialize")
        if language not in SUPPORTED_LANGUAGES:
            raise ValidationError(f"Language '{language}' not supported by EasyOCR")

        try:
            import io  # noqa: PLC0415
            import numpy as np  # noqa: PLC0415  # type: ignore[import-not-found]
            from PIL import Image  # noqa: PLC0415

            image = Image.open(io.BytesIO(image_bytes))
            width, height = image.size
            result = self._reader.readtext(np.array(image), beamWidth=self.beam_width)
            content, confidence, text_regions = self._process_easyocr_result(result)
            return {
                "content": content,
                "metadata": {"width": width, "height": height, "confidence": confidence, "text_regions": text_regions},
            }
        except Exception as e:
            raise OcrError(f"EasyOCR processing failed: {e}") from e

    def process_image_file(self, path: str, language: str) -> dict[str, Any]:
        from pathlib import Path  # noqa: PLC0415
        return self.process_image(Path(path).read_bytes(), language)

    @staticmethod
    def _process_easyocr_result(result: list[Any]) -> tuple[str, float, int]:
        if not result:
            return "", 0.0, 0
        if all(len(item) == 2 for item in result):
            parts, total = [], 0.0
            for text, conf in result:
                if text:
                    parts.append(text)
                total += conf
            return "\n".join(parts), total / len(result) if result else 0.0, len(result)

        sorted_results = sorted(result, key=lambda x: x[0][0][1] + x[0][2][1])
        lines: list[list[Any]] = []
        current: list[Any] = []
        prev_y: float | None = None
        for item in sorted_results:
            y = sum(p[1] for p in item[0]) / 4
            if prev_y is None or abs(y - prev_y) > 20:
                if current:
                    lines.append(current)
                current = [item]
            else:
                current.append(item)
            prev_y = y
        if current:
            lines.append(current)

        parts, total, count = [], 0.0, 0
        for line in lines:
            line_sorted = sorted(line, key=lambda x: x[0][0][0])
            texts = []
            for _, text, conf in line_sorted:
                if text:
                    texts.append(text)
                    total += conf
                    count += 1
            if texts:
                parts.append(" ".join(texts))
        return "\n".join(parts), total / count if count else 0.0, count

    @staticmethod
    def _is_cuda_available() -> bool:
        try:
            import torch  # noqa: PLC0415
            return bool(torch.cuda.is_available())
        except ImportError:
            return False

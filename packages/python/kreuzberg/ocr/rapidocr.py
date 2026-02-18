"""RapidOCR backend for document OCR processing."""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Any

from kreuzberg.exceptions import OCRError, ValidationError

logger = logging.getLogger(__name__)

_LANGUAGE_ALIASES = {
    "en": "en",
    "eng": "en",
    "ch": "ch",
    "zh": "ch",
    "zho": "ch",
    "chi": "ch",
    "jpn": "japan",
    "ja": "japan",
    "kor": "korean",
    "ko": "korean",
    "ara": "arabic",
    "ar": "arabic",
    "ell": "el",
    "el": "el",
    "tha": "th",
    "th": "th",
    "tam": "ta",
    "ta": "ta",
    "tel": "te",
    "te": "te",
    "rus": "cyrillic",
    "ru": "cyrillic",
    "ukr": "cyrillic",
    "uk": "cyrillic",
    "deu": "latin",
    "de": "latin",
    "fra": "latin",
    "fr": "latin",
    "spa": "latin",
    "es": "latin",
    "ita": "latin",
    "it": "latin",
    "por": "latin",
    "pt": "latin",
}

_RAPIDOCR_LANGS = {
    "ch",
    "ch_doc",
    "en",
    "arabic",
    "chinese_cht",
    "cyrillic",
    "devanagari",
    "japan",
    "korean",
    "ka",
    "latin",
    "ta",
    "te",
    "eslav",
    "th",
    "el",
}


class RapidOCRBackend:
    """RapidOCR backend for OCR processing."""

    def __init__(
        self,
        *,
        language: str = "en",
        config_path: str | None = None,
        params: dict[str, Any] | None = None,
    ) -> None:
        try:
            import rapidocr as rapidocr_module  # noqa: PLC0415
        except ImportError as e:
            msg = "RapidOCR support requires the 'rapidocr' package. Install with: pip install \"kreuzberg[rapidocr]\""
            raise ImportError(msg) from e

        self._rapidocr_module = rapidocr_module
        self.config_path = config_path
        self.base_params = params or {}
        self.default_language = self._normalize_language(language)
        self._engines: dict[str, Any] = {}

    def name(self) -> str:
        """Return backend identifier used by Kreuzberg OCR config."""
        return "rapid-ocr"

    def supported_languages(self) -> list[str]:
        """Return normalized and alias language codes accepted by this backend."""
        return sorted(set(_LANGUAGE_ALIASES.keys()) | _RAPIDOCR_LANGS)

    def initialize(self) -> None:
        """Eagerly initialize the default language OCR engine."""
        self._get_engine(self.default_language)

    def shutdown(self) -> None:
        """Release cached RapidOCR engine instances."""
        self._engines.clear()

    def process_image(self, image_bytes: bytes, language: str) -> dict[str, Any]:
        """Run OCR for raw image bytes and return Kreuzberg's backend response shape."""
        normalized_language = self._normalize_language(language)
        engine = self._get_engine(normalized_language)

        try:
            result = engine(image_bytes)
            raw_boxes = getattr(result, "boxes", None)
            boxes = self._normalize_boxes(raw_boxes)
            txts = list(getattr(result, "txts", []) or [])
            scores = list(getattr(result, "scores", []) or [])
            ocr_elements = self._build_ocr_elements(boxes, txts, scores)

            content = "\n".join(str(txt).strip() for txt in txts if str(txt).strip())
            confidence = sum(float(score) for score in scores) / len(scores) if scores else 0.0

            metadata: dict[str, Any] = {
                "backend": "rapid-ocr",
                "language": normalized_language,
                "confidence": confidence,
                "text_regions": max(len(txts), len(boxes)),
            }
            if boxes:
                metadata["boxes"] = boxes

            img = getattr(result, "img", None)
            if img is not None and hasattr(img, "shape") and len(img.shape) >= 2:
                metadata["height"] = int(img.shape[0])
                metadata["width"] = int(img.shape[1])

            return {
                "content": content,
                "metadata": metadata,
                "tables": [],
                "ocr_elements": ocr_elements,
            }
        except Exception as e:
            msg = f"RapidOCR processing failed: {e}"
            raise OCRError(msg) from e

    def process_file(self, path: str, language: str) -> dict[str, Any]:
        """Run OCR for an image path by delegating to ``process_image``."""
        image_bytes = Path(path).read_bytes()
        return self.process_image(image_bytes, language)

    def _normalize_language(self, language: str) -> str:
        normalized = _LANGUAGE_ALIASES.get(language.lower(), language.lower())
        if normalized not in _RAPIDOCR_LANGS:
            msg = f"Language '{language}' not supported by RapidOCR"
            raise ValidationError(
                msg,
                context={
                    "language": language,
                    "normalized_language": normalized,
                    "supported_languages": sorted(self.supported_languages()),
                },
            )
        return normalized

    @staticmethod
    def _normalize_boxes(raw_boxes: Any) -> list[list[list[float]]]:
        """Convert RapidOCR box output into JSON-serializable quadrilateral points."""
        normalized: list[list[list[float]]] = []

        try:
            boxes = list(raw_boxes)
        except TypeError:
            return normalized

        for raw_box in boxes:
            box = raw_box.tolist() if hasattr(raw_box, "tolist") else raw_box

            try:
                points = list(box)
            except TypeError:
                continue

            quad: list[list[float]] = []
            for raw_point in points:
                point = raw_point.tolist() if hasattr(raw_point, "tolist") else raw_point
                if not isinstance(point, (list, tuple)) or len(point) < 2:
                    continue
                try:
                    quad.append([float(point[0]), float(point[1])])
                except (TypeError, ValueError):
                    continue

            if len(quad) >= 4:
                normalized.append(quad[:4])

        return normalized

    @staticmethod
    def _build_ocr_elements(
        boxes: list[list[list[float]]],
        txts: list[str],
        scores: list[float],
    ) -> list[dict[str, Any]]:
        """Build Kreuzberg OCR elements with quadrilateral geometry from RapidOCR output."""
        elements: list[dict[str, Any]] = []

        for idx, box in enumerate(boxes):
            text = str(txts[idx]).strip() if idx < len(txts) else ""
            if not text:
                continue

            recognition = float(scores[idx]) if idx < len(scores) else 0.0
            points = [[round(x), round(y)] for x, y in box[:4]]
            if len(points) < 4:
                continue

            elements.append(
                {
                    "text": text,
                    "geometry": {"type": "quadrilateral", "points": points},
                    "confidence": {"recognition": recognition},
                    "level": "line",
                    "page_number": 1,
                }
            )

        return elements

    def _get_engine(self, language: str) -> Any:
        if language in self._engines:
            return self._engines[language]

        try:
            det_language = "en" if language == "en" else "ch" if language == "ch" else "multi"

            params = dict(self.base_params)
            params["Det.lang_type"] = self._rapidocr_module.LangDet(det_language)
            params["Cls.lang_type"] = self._rapidocr_module.LangCls.CH
            params["Rec.lang_type"] = self._rapidocr_module.LangRec(language)

            engine = self._rapidocr_module.RapidOCR(config_path=self.config_path, params=params)
            self._engines[language] = engine
            logger.info("Initialized RapidOCR engine for language=%s", language)
            return engine
        except Exception as e:
            msg = f"Failed to initialize RapidOCR engine for language '{language}': {e}"
            raise OCRError(msg) from e

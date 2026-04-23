"""Tests for kreuzberg/ocr/easyocr.py."""

from __future__ import annotations

from unittest.mock import Mock, patch

import pytest

from kreuzberg.exceptions import OcrError, ValidationError


def test_easyocr_import_error() -> None:
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    with patch.dict("sys.modules", {"easyocr": None}):
        with pytest.raises(ImportError, match="kreuzberg\\[easyocr\\]"):
            EasyOCRBackend()


def test_easyocr_unsupported_language() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    with pytest.raises(ValidationError, match="Unsupported"):
        EasyOCRBackend(languages=["invalid_lang"])


def test_easyocr_initialize_idempotent() -> None:
    easyocr = pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    with patch.object(easyocr, "Reader", return_value=Mock()):
        backend = EasyOCRBackend(languages=["en"], use_gpu=False)
        backend.initialize()
        first = backend._reader
        backend.initialize()
        assert first is backend._reader
        easyocr.Reader.assert_called_once()


def test_easyocr_initialize_failure() -> None:
    easyocr = pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    with patch.object(easyocr, "Reader", side_effect=RuntimeError("fail")):
        backend = EasyOCRBackend(languages=["en"], use_gpu=False)
        with pytest.raises(OcrError, match="Failed to initialize"):
            backend.initialize()


def test_easyocr_process_image_reader_none() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    backend = EasyOCRBackend(languages=["en"], use_gpu=False)
    with patch.object(backend, "initialize"):
        backend._reader = None
        with pytest.raises(RuntimeError, match="failed to initialize"):
            backend.process_image(b"fake", "en")


def test_easyocr_process_image_unsupported_language() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    backend = EasyOCRBackend(languages=["en"], use_gpu=False)
    backend._reader = Mock()
    with pytest.raises(ValidationError, match="not supported"):
        backend.process_image(b"fake", "invalid_lang")


def test_process_easyocr_result_empty() -> None:
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    assert EasyOCRBackend._process_easyocr_result([]) == ("", 0.0, 0)


def test_process_easyocr_result_two_item() -> None:
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    result = [("Hello", 0.95), ("World", 0.90), ("", 0.85)]
    content, conf, n = EasyOCRBackend._process_easyocr_result(result)
    assert content == "Hello\nWorld"
    # Confidence is averaged over all items (including empty text)
    assert conf == pytest.approx((0.95 + 0.90 + 0.85) / 3)
    assert n == 3


def test_process_easyocr_result_line_grouping() -> None:
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    result = [
        ([[0, 10], [50, 10], [50, 30], [0, 30]], "Hello", 0.95),
        ([[60, 10], [110, 10], [110, 30], [60, 30]], "World", 0.90),
        ([[0, 50], [50, 50], [50, 70], [0, 70]], "Second", 0.88),
        ([[60, 50], [110, 50], [110, 70], [60, 70]], "Line", 0.87),
    ]
    content, conf, n = EasyOCRBackend._process_easyocr_result(result)
    assert "Hello World" in content
    assert "Second Line" in content
    assert n == 4


def test_easyocr_name() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    assert EasyOCRBackend(languages=["en"], use_gpu=False).name() == "easyocr"


def test_easyocr_supported_languages() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    langs = EasyOCRBackend(languages=["en"], use_gpu=False).supported_languages()
    assert isinstance(langs, list)
    assert langs == sorted(langs)
    assert "en" in langs


def test_easyocr_no_document_processing() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    assert EasyOCRBackend(languages=["en"], use_gpu=False).supports_document_processing() is False


def test_easyocr_shutdown() -> None:
    pytest.importorskip("easyocr", reason="EasyOCR not installed")
    from kreuzberg.ocr.easyocr import EasyOCRBackend
    backend = EasyOCRBackend(languages=["en"], use_gpu=False)
    backend._reader = Mock()
    backend.shutdown()
    assert backend._reader is None

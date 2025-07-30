from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from kreuzberg._extractors._image import ImageExtractor
from kreuzberg._types import ExtractionConfig, ExtractionResult
from kreuzberg.exceptions import ValidationError

if TYPE_CHECKING:
    from collections.abc import Generator


@pytest.fixture
def extractor() -> ImageExtractor:
    config = ExtractionConfig(ocr_backend="tesseract")
    return ImageExtractor(mime_type="image/png", config=config)


@pytest.fixture
def mock_ocr_backend() -> Generator[MagicMock, None, None]:
    backend = MagicMock()
    backend.process_file = AsyncMock()
    backend.process_file_sync = MagicMock()
    backend.process_batch_sync = MagicMock()

    with patch("kreuzberg._extractors._image.get_ocr_backend", return_value=backend):
        yield backend


@pytest.mark.anyio
async def test_extract_path_async_no_ocr_backend() -> None:
    config = ExtractionConfig(ocr_backend=None)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    with pytest.raises(ValidationError) as excinfo:
        await extractor.extract_path_async(Path("dummy_path"))

    assert "ocr_backend is None" in str(excinfo.value)


@pytest.mark.anyio
async def test_extract_path_async(mock_ocr_backend: MagicMock, tmp_path: Path) -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    image_path = tmp_path / "test.png"
    image_path.write_bytes(b"dummy image content")

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )
    mock_ocr_backend.process_file.return_value = expected_result

    result = await extractor.extract_path_async(image_path)

    mock_ocr_backend.process_file.assert_called_once()
    assert result == expected_result


def test_extract_path_sync(mock_ocr_backend: MagicMock, tmp_path: Path) -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    image_path = tmp_path / "test.png"
    image_path.write_bytes(b"dummy image content")

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )
    mock_ocr_backend.process_file_sync.return_value = expected_result

    result = extractor.extract_path_sync(image_path)

    mock_ocr_backend.process_file_sync.assert_called_once()
    assert result == expected_result


def test_extract_bytes_sync(mock_ocr_backend: MagicMock) -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )

    with patch.object(extractor, "extract_path_sync") as mock_extract_path:
        mock_extract_path.return_value = expected_result
        result = extractor.extract_bytes_sync(b"dummy image content")

        mock_extract_path.assert_called_once()
        assert result == expected_result


@pytest.mark.parametrize(
    "mime_type,expected_extension",
    [
        ("image/png", "png"),
        ("image/jpeg", "jpg"),
        ("image/gif", "gif"),
        ("image/bmp", "bmp"),
        ("image/tiff", "tiff"),
        ("image/webp", "webp"),
    ],
)
def test_get_extension_from_mime_type(mime_type: str, expected_extension: str) -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type=mime_type, config=config)

    extension = extractor._get_extension_from_mime_type(mime_type)
    assert extension == expected_extension


def test_get_extension_from_partial_mime_type() -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/jpeg", config=config)

    extension = extractor._get_extension_from_mime_type("image")
    assert extension == "bmp"


def test_get_extension_from_unsupported_mime_type() -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    with pytest.raises(ValidationError) as excinfo:
        extractor._get_extension_from_mime_type("application/unsupported")

    assert "unsupported mimetype" in str(excinfo.value)
    assert "application/unsupported" in str(excinfo.value)


@pytest.mark.anyio
async def test_extract_bytes_async(mock_ocr_backend: MagicMock) -> None:
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )
    mock_ocr_backend.process_file.return_value = expected_result

    mock_path = MagicMock()
    mock_unlink = AsyncMock()

    with patch("kreuzberg._extractors._image.create_temp_file") as mock_create_temp:
        mock_create_temp.return_value = (mock_path, mock_unlink)

        with patch("kreuzberg._extractors._image.AsyncPath") as mock_async_path:
            mock_async_path_instance = MagicMock()
            mock_async_path_instance.write_bytes = AsyncMock()
            mock_async_path.return_value = mock_async_path_instance

            result = await extractor.extract_bytes_async(b"dummy image content")

            mock_create_temp.assert_called_once_with(".png")

            mock_async_path_instance.write_bytes.assert_called_once_with(b"dummy image content")

            mock_ocr_backend.process_file.assert_called_once_with(mock_path, **config.get_config_dict())

            mock_unlink.assert_called_once()

            assert result == expected_result


def test_ocr_backend_none_validation_error() -> None:
    """Test validation error when OCR backend is None."""
    config = ExtractionConfig(ocr_backend=None)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    with pytest.raises(ValidationError, match="ocr_backend is None"):
        extractor.extract_path_sync(Path("dummy.png"))


@pytest.mark.anyio
async def test_ocr_backend_none_validation_error_async() -> None:
    """Test validation error when OCR backend is None in async mode."""
    config = ExtractionConfig(ocr_backend=None)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    with pytest.raises(ValidationError, match="ocr_backend is None"):
        await extractor.extract_path_async(Path("dummy.png"))


def test_extract_bytes_temp_file_cleanup_on_error() -> None:
    """Test that temp files are cleaned up even when processing fails."""
    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    with patch("tempfile.mkstemp") as mock_mkstemp:
        mock_fd = 42
        mock_temp_path = "/tmp/test_image.png"
        mock_mkstemp.return_value = (mock_fd, mock_temp_path)

        with patch("os.fdopen") as mock_fdopen:
            mock_file = MagicMock()
            mock_fdopen.return_value.__enter__.return_value = mock_file

            with patch.object(extractor, "extract_path_sync") as mock_extract:
                mock_extract.side_effect = Exception("Processing failed")

                with patch("pathlib.Path.unlink") as mock_unlink:
                    with pytest.raises(Exception, match="Processing failed"):
                        extractor.extract_bytes_sync(b"image data")

                    # Verify temp file was still cleaned up
                    mock_unlink.assert_called_once()


def test_extract_path_sync_no_ocr_backend() -> None:
    """Test sync path extraction when ocr_backend is None."""
    config = ExtractionConfig(ocr_backend=None)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    with pytest.raises(ValidationError) as excinfo:
        extractor.extract_path_sync(Path("dummy_path"))

    assert "ocr_backend is None" in str(excinfo.value)


def test_extract_path_sync_with_tesseract_config(mock_ocr_backend: MagicMock) -> None:
    """Test sync path extraction with TesseractConfig."""
    from kreuzberg._ocr._tesseract import TesseractConfig

    tesseract_config = TesseractConfig()
    config = ExtractionConfig(ocr_backend="tesseract", ocr_config=tesseract_config)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    image_path = Path("test.png")

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )
    mock_ocr_backend.process_file_sync.return_value = expected_result

    result = extractor.extract_path_sync(image_path)

    mock_ocr_backend.process_file_sync.assert_called_once()
    assert result == expected_result


def test_extract_path_sync_with_paddleocr_config(mock_ocr_backend: MagicMock) -> None:
    """Test sync path extraction with PaddleOCRConfig."""
    from kreuzberg._ocr._paddleocr import PaddleOCRConfig

    paddle_config = PaddleOCRConfig()
    config = ExtractionConfig(ocr_backend="paddleocr", ocr_config=paddle_config)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    image_path = Path("test.png")

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )
    mock_ocr_backend.process_file_sync.return_value = expected_result

    result = extractor.extract_path_sync(image_path)

    mock_ocr_backend.process_file_sync.assert_called_once()
    assert result == expected_result


def test_extract_path_sync_with_easyocr_config(mock_ocr_backend: MagicMock) -> None:
    """Test sync path extraction with EasyOCRConfig."""
    from kreuzberg._ocr._easyocr import EasyOCRConfig

    easy_config = EasyOCRConfig()
    config = ExtractionConfig(ocr_backend="easyocr", ocr_config=easy_config)
    extractor = ImageExtractor(mime_type="image/png", config=config)

    image_path = Path("test.png")

    expected_result = ExtractionResult(
        content="extracted text", chunks=[], mime_type="text/plain", metadata={"quality_score": 1.0}
    )
    mock_ocr_backend.process_file_sync.return_value = expected_result

    result = extractor.extract_path_sync(image_path)

    mock_ocr_backend.process_file_sync.assert_called_once()
    assert result == expected_result


# Integration Tests - These test with real images and OCR
@pytest.mark.anyio
async def test_extract_real_image_integration() -> None:
    """Integration test with real image and OCR."""
    test_image_path = Path(__file__).parent.parent / "test_source_files" / "images" / "test_hello_world.png"
    if not test_image_path.exists():
        pytest.skip("Test image not found")

    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    result = await extractor.extract_path_async(test_image_path)

    assert isinstance(result, ExtractionResult)
    assert result.mime_type == "text/plain"
    assert len(result.content) > 0  # Should extract some text


def test_extract_real_image_sync_integration() -> None:
    """Integration test with real image and OCR (sync)."""
    test_image_path = Path(__file__).parent.parent / "test_source_files" / "images" / "test_hello_world.png"
    if not test_image_path.exists():
        pytest.skip("Test image not found")

    config = ExtractionConfig(ocr_backend="tesseract")
    extractor = ImageExtractor(mime_type="image/png", config=config)

    result = extractor.extract_path_sync(test_image_path)

    assert isinstance(result, ExtractionResult)
    assert result.mime_type == "text/plain"
    assert len(result.content) > 0  # Should extract some text

from __future__ import annotations

from typing import TYPE_CHECKING, Any
from unittest.mock import Mock, patch

import numpy as np
import pytest
from PIL import Image

from kreuzberg._ocr._paddleocr import PaddleBackend
from kreuzberg._types import ExtractionResult
from kreuzberg.exceptions import MissingDependencyError, OCRError

if TYPE_CHECKING:
    from pathlib import Path

    from pytest_mock import MockerFixture


@pytest.fixture
def backend() -> PaddleBackend:
    return PaddleBackend()


@pytest.fixture
def mock_paddleocr(mocker: MockerFixture) -> Mock:
    """Mock the PaddleOCR class."""
    mock = mocker.patch("kreuzberg._ocr._paddleocr.PaddleOCR")
    instance = mock.return_value

    instance.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    return mock


@pytest.fixture
def mock_run_sync(mocker: MockerFixture) -> Mock:
    """Mock the run_sync function."""

    async def mock_async_run_sync(func: Any, *args: Any, **kwargs: Any) -> Any:
        if isinstance(func, Mock) and kwargs.get("image_np") is not None:
            return [
                [
                    [
                        [[10, 10], [100, 10], [100, 30], [10, 30]],
                        ("Sample text 1", 0.95),
                    ],
                    [
                        [[10, 40], [100, 40], [100, 60], [10, 60]],
                        ("Sample text 2", 0.90),
                    ],
                ]
            ]

        if callable(func) and hasattr(func, "__name__") and func.__name__ == "open":
            img = Mock(spec=Image.Image)
            img.size = (100, 100)

            array_interface = {
                "shape": (100, 100, 3),
                "typestr": "|u1",
                "data": np.zeros((100, 100, 3), dtype=np.uint8).tobytes(),
                "strides": None,
                "version": 3,
            }
            type(img).__array_interface__ = array_interface
            return img

        if callable(func) and hasattr(func, "__name__") and func.__name__ == "PaddleOCR":
            paddle_ocr = Mock()
            paddle_ocr.ocr = Mock()
            paddle_ocr.ocr.return_value = [
                [
                    [
                        [[10, 10], [100, 10], [100, 30], [10, 30]],
                        ("Sample text 1", 0.95),
                    ],
                    [
                        [[10, 40], [100, 40], [100, 60], [10, 60]],
                        ("Sample text 2", 0.90),
                    ],
                ]
            ]
            return paddle_ocr
        return func(*args, **kwargs)

    return mocker.patch("kreuzberg._ocr._paddleocr.run_sync", side_effect=mock_async_run_sync)


@pytest.fixture
def mock_find_spec(mocker: MockerFixture) -> Mock:
    """Mock the find_spec function to simulate PaddleOCR installation."""
    mock = mocker.patch("kreuzberg._ocr._paddleocr.find_spec")
    mock.return_value = True
    return mock


@pytest.fixture
def mock_find_spec_missing(mocker: MockerFixture) -> Mock:
    """Mock the find_spec function to simulate missing PaddleOCR installation."""
    mock = mocker.patch("kreuzberg._ocr._paddleocr.find_spec")
    mock.return_value = None
    return mock


@pytest.fixture
def mock_image() -> Mock:
    """Create a mock PIL Image with numpy array interface."""
    img = Mock(spec=Image.Image)
    img.size = (100, 100)

    array_interface = {
        "shape": (100, 100, 3),
        "typestr": "|u1",
        "data": np.zeros((100, 100, 3), dtype=np.uint8).tobytes(),
        "strides": None,
        "version": 3,
    }
    type(img).__array_interface__ = array_interface
    return img


@pytest.mark.anyio
async def test_normalize_language() -> None:
    """Test the language normalization function."""

    assert PaddleBackend._normalize_language("eng") == "en"
    assert PaddleBackend._normalize_language("chi") == "ch"
    assert PaddleBackend._normalize_language("fra") == "french"
    assert PaddleBackend._normalize_language("deu") == "german"
    assert PaddleBackend._normalize_language("kor") == "korean"
    assert PaddleBackend._normalize_language("jpn") == "japan"

    assert PaddleBackend._normalize_language("en") == "en"
    assert PaddleBackend._normalize_language("zh") == "ch"
    assert PaddleBackend._normalize_language("fr") == "french"
    assert PaddleBackend._normalize_language("de") == "german"
    assert PaddleBackend._normalize_language("ko") == "korean"
    assert PaddleBackend._normalize_language("ja") == "japan"

    assert PaddleBackend._normalize_language("french") == "french"
    assert PaddleBackend._normalize_language("german") == "german"
    assert PaddleBackend._normalize_language("korean") == "korean"
    assert PaddleBackend._normalize_language("japan") == "japan"

    assert PaddleBackend._normalize_language("unknown") == "en"
    assert PaddleBackend._normalize_language("spa") == "en"


@pytest.mark.anyio
async def test_is_mkldnn_supported(mocker: MockerFixture) -> None:
    """Test the MKL-DNN support detection function."""

    mocker.patch("platform.system", return_value="Linux")
    mocker.patch("platform.processor", return_value="x86_64")
    mocker.patch("platform.machine", return_value="x86_64")
    assert PaddleBackend._is_mkldnn_supported() is True

    mocker.patch("platform.system", return_value="Windows")
    mocker.patch("platform.processor", return_value="Intel64 Family 6")
    assert PaddleBackend._is_mkldnn_supported() is True

    mocker.patch("platform.system", return_value="Darwin")
    mocker.patch("platform.machine", return_value="x86_64")
    assert PaddleBackend._is_mkldnn_supported() is True

    mocker.patch("platform.system", return_value="Darwin")
    mocker.patch("platform.machine", return_value="arm64")
    assert PaddleBackend._is_mkldnn_supported() is False

    mocker.patch("platform.system", return_value="FreeBSD")
    assert PaddleBackend._is_mkldnn_supported() is False


@pytest.mark.anyio
async def test_init_paddle_ocr(
    backend: PaddleBackend, mock_paddleocr: Mock, mock_run_sync: Mock, mock_find_spec: Mock
) -> None:
    """Test initializing PaddleOCR."""

    PaddleBackend._paddle_ocr = None

    await backend._init_paddle_ocr()

    mock_run_sync.assert_called_once()
    mock_paddleocr.assert_called_once()

    assert PaddleBackend._paddle_ocr is not None

    mock_run_sync.reset_mock()
    mock_paddleocr.reset_mock()

    await backend._init_paddle_ocr()
    mock_run_sync.assert_not_called()
    mock_paddleocr.assert_not_called()


@pytest.mark.anyio
async def test_init_paddle_ocr_with_language(
    backend: PaddleBackend, mock_paddleocr: Mock, mock_run_sync: Mock, mock_find_spec: Mock
) -> None:
    """Test initializing PaddleOCR with a specific language."""

    PaddleBackend._paddle_ocr = None

    await backend._init_paddle_ocr(language="fra")

    call_kwargs = mock_run_sync.call_args[1]
    assert call_kwargs.get("lang") == "french"


@pytest.mark.anyio
async def test_init_paddle_ocr_missing_dependency(backend: PaddleBackend, mock_find_spec_missing: Mock) -> None:
    """Test initializing PaddleOCR when the dependency is missing."""

    PaddleBackend._paddle_ocr = None

    with patch("kreuzberg._ocr._paddleocr.run_sync", side_effect=ImportError("No module named 'paddleocr'")):
        with pytest.raises(MissingDependencyError) as excinfo:
            await backend._init_paddle_ocr()

        assert "PaddleOCR is not installed" in str(excinfo.value)


@pytest.mark.anyio
async def test_init_paddle_ocr_initialization_error(backend: PaddleBackend, mock_find_spec: Mock) -> None:
    """Test handling initialization errors."""

    PaddleBackend._paddle_ocr = None

    with patch("kreuzberg._ocr._paddleocr.run_sync", side_effect=Exception("Initialization error")):
        with pytest.raises(OCRError) as excinfo:
            await backend._init_paddle_ocr()

        assert "Failed to initialize PaddleOCR" in str(excinfo.value)


@pytest.mark.anyio
async def test_process_image(
    backend: PaddleBackend, mock_image: Mock, mock_run_sync: Mock, mock_paddleocr: Mock
) -> None:
    """Test processing an image."""

    paddle_mock = Mock()

    paddle_mock.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    PaddleBackend._paddle_ocr = paddle_mock

    result = await backend.process_image(mock_image)

    assert isinstance(result, ExtractionResult)
    assert "Sample text 1 Sample text 2" in result.content
    assert result.mime_type == "text/plain"
    assert result.metadata.get("width") == 100
    assert result.metadata.get("height") == 100
    assert result.metadata.get("created_at") is not None


@pytest.mark.anyio
async def test_process_image_with_options(backend: PaddleBackend, mock_image: Mock, mock_run_sync: Mock) -> None:
    """Test processing an image with custom options."""

    paddle_mock = Mock()

    paddle_mock.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    PaddleBackend._paddle_ocr = paddle_mock

    result = await backend.process_image(
        mock_image,
        language="german",
        use_angle_cls=True,
        det_db_thresh=0.4,
        det_db_box_thresh=0.6,
    )

    assert isinstance(result, ExtractionResult)
    assert "Sample text 1 Sample text 2" in result.content


@pytest.mark.anyio
async def test_process_image_error(backend: PaddleBackend, mock_image: Mock) -> None:
    """Test handling errors during image processing."""

    paddle_mock = Mock()

    paddle_mock.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    PaddleBackend._paddle_ocr = paddle_mock

    with patch("kreuzberg._ocr._paddleocr.run_sync", side_effect=Exception("OCR processing error")):
        with pytest.raises(OCRError) as excinfo:
            await backend.process_image(mock_image)

        assert "Failed to OCR using PaddleOCR" in str(excinfo.value)


@pytest.mark.anyio
async def test_process_file(backend: PaddleBackend, mock_run_sync: Mock, ocr_image: Path) -> None:
    """Test processing a file."""

    paddle_mock = Mock()

    paddle_mock.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    PaddleBackend._paddle_ocr = paddle_mock

    result = await backend.process_file(ocr_image)

    assert isinstance(result, ExtractionResult)
    assert "Sample text 1 Sample text 2" in result.content


@pytest.mark.anyio
async def test_process_file_with_options(backend: PaddleBackend, mock_run_sync: Mock, ocr_image: Path) -> None:
    """Test processing a file with custom options."""

    paddle_mock = Mock()

    paddle_mock.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    PaddleBackend._paddle_ocr = paddle_mock

    result = await backend.process_file(
        ocr_image,
        language="french",
        use_angle_cls=True,
        det_db_thresh=0.4,
    )

    assert isinstance(result, ExtractionResult)
    assert "Sample text 1 Sample text 2" in result.content


@pytest.mark.anyio
async def test_process_file_error(backend: PaddleBackend, ocr_image: Path) -> None:
    """Test handling errors during file processing."""

    paddle_mock = Mock()

    paddle_mock.ocr.return_value = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Sample text 1", 0.95),
            ],
            [
                [[10, 40], [100, 40], [100, 60], [10, 60]],
                ("Sample text 2", 0.90),
            ],
        ]
    ]
    PaddleBackend._paddle_ocr = paddle_mock

    with patch("kreuzberg._ocr._paddleocr.run_sync", side_effect=Exception("File processing error")):
        with pytest.raises(OCRError) as excinfo:
            await backend.process_file(ocr_image)

        assert "Failed to load or process image using PaddleOCR" in str(excinfo.value)


@pytest.mark.anyio
async def test_process_paddle_result_empty() -> None:
    """Test processing an empty PaddleOCR result."""

    image = Mock(spec=Image.Image)
    image.size = (100, 100)

    result = PaddleBackend._process_paddle_result([], image)

    assert isinstance(result, ExtractionResult)
    assert result.content == ""

    assert isinstance(result.metadata, dict)
    assert result.metadata.get("width") == 100
    assert result.metadata.get("height") == 100


@pytest.mark.anyio
async def test_process_paddle_result_complex() -> None:
    """Test processing a complex PaddleOCR result with multiple lines."""

    image = Mock(spec=Image.Image)
    image.size = (200, 200)

    paddle_result = [
        [
            [
                [[10, 10], [100, 10], [100, 30], [10, 30]],
                ("Line 1 Text 1", 0.95),
            ],
            [
                [[110, 10], [200, 10], [200, 30], [110, 30]],
                ("Line 1 Text 2", 0.90),
            ],
            [
                [[10, 50], [100, 50], [100, 70], [10, 70]],
                ("Line 2 Text 1", 0.85),
            ],
            [
                [[110, 50], [200, 50], [200, 70], [110, 70]],
                ("Line 2 Text 2", 0.80),
            ],
            [
                [[10, 90], [200, 90], [200, 110], [10, 110]],
                ("Line 3 Text", 0.75),
            ],
        ]
    ]

    result = PaddleBackend._process_paddle_result(paddle_result, image)

    assert isinstance(result, ExtractionResult)
    assert "Line 1 Text 1 Line 1 Text 2" in result.content
    assert "Line 2 Text 1 Line 2 Text 2" in result.content
    assert "Line 3 Text" in result.content

    assert isinstance(result.metadata, dict)
    assert result.metadata.get("width") == 200
    assert result.metadata.get("height") == 200


@pytest.mark.anyio
async def test_integration_process_file(backend: PaddleBackend, ocr_image: Path) -> None:
    """Integration test for processing a file with actual PaddleOCR."""

    try:
        from paddleocr import PaddleOCR  # noqa: F401
    except ImportError:
        pytest.skip("PaddleOCR not installed")

    import platform

    if platform.system() == "Darwin" and platform.machine() == "arm64":
        pytest.skip("Test not applicable on Mac M1/ARM architecture")

    try:
        result = await backend.process_file(ocr_image)
        assert isinstance(result, ExtractionResult)
        assert result.content.strip()
    except (MissingDependencyError, OCRError):
        pytest.skip("PaddleOCR not properly installed or configured")


@pytest.mark.anyio
async def test_integration_process_image(backend: PaddleBackend, ocr_image: Path) -> None:
    """Integration test for processing an image with actual PaddleOCR."""

    try:
        from paddleocr import PaddleOCR  # noqa: F401
    except ImportError:
        pytest.skip("PaddleOCR not installed")

    import platform

    if platform.system() == "Darwin" and platform.machine() == "arm64":
        pytest.skip("Test not applicable on Mac M1/ARM architecture")

    try:
        image = Image.open(ocr_image)
        with image:
            result = await backend.process_image(image)
            assert isinstance(result, ExtractionResult)
            assert result.content.strip()
    except (MissingDependencyError, OCRError):
        pytest.skip("PaddleOCR not properly installed or configured")

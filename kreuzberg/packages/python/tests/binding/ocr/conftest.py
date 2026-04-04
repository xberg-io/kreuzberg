"""Pytest configuration for OCR backend tests.

Provides mock implementations of optional OCR libraries (easyocr)
to allow tests to run without requiring the actual libraries to be installed.
"""

from __future__ import annotations

import sys
from typing import TYPE_CHECKING
from unittest.mock import MagicMock, Mock

import pytest

if TYPE_CHECKING:
    from collections.abc import Generator


@pytest.fixture(scope="session", autouse=True)
def mock_ocr_libraries() -> Generator[None, None, None]:
    """Mock easyocr module to allow tests to run without it installed.

    This fixture is automatically used for all tests in this module.
    It injects mock modules into sys.modules so that pytest.importorskip() will pass
    and allow the OCR backend tests to run with mocks.
    """
    # Create mock easyocr module with necessary attributes
    easyocr_mock = MagicMock()
    easyocr_mock.Reader = Mock()
    sys.modules["easyocr"] = easyocr_mock

    # Create mock torch module (used by easyocr for CUDA detection)
    torch_mock = MagicMock()
    torch_mock.cuda = MagicMock()
    torch_mock.cuda.is_available = Mock(return_value=False)
    sys.modules["torch"] = torch_mock

    yield

    # Clean up mocks after session
    sys.modules.pop("easyocr", None)
    sys.modules.pop("torch", None)

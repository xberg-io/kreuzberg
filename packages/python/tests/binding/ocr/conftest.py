"""Mock easyocr/torch for tests that don't need real OCR."""

from __future__ import annotations

import sys
from typing import TYPE_CHECKING
from unittest.mock import MagicMock, Mock

import pytest

if TYPE_CHECKING:
    from collections.abc import Generator


@pytest.fixture(scope="session", autouse=True)
def mock_ocr_libraries() -> Generator[None, None, None]:
    easyocr_mock = MagicMock()
    easyocr_mock.Reader = Mock()
    sys.modules["easyocr"] = easyocr_mock

    torch_mock = MagicMock()
    torch_mock.cuda = MagicMock()
    torch_mock.cuda.is_available = Mock(return_value=False)
    sys.modules["torch"] = torch_mock

    yield

    sys.modules.pop("easyocr", None)
    sys.modules.pop("torch", None)

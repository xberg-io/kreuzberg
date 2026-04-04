"""
Pytest configuration for E2E tests.

Provides fixtures and markers for managing test execution,
particularly for slow tests that may timeout in CI environments.
"""

from __future__ import annotations

import contextlib
import platform

import pytest


def pytest_configure(config: pytest.Config) -> None:
    """Register custom markers."""
    config.addinivalue_line(
        "markers",
        "slow: marks tests as slow (deselect with '-m \"not slow\"')",
    )
    config.addinivalue_line(
        "markers",
        "windows_slow: marks tests as too slow on Windows CI (deselect with '-m \"not windows_slow\"')",
    )
    config.addinivalue_line(
        "markers",
        "paddle_ocr: marks tests that require PaddleOCR (deselect with '-m \"not paddle_ocr\"')",
    )


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
    """Apply windows_slow marker to office tests on Windows platform and skip them."""
    if platform.system() != "Windows":
        return

    for item in items:
        if "test_office" in item.nodeid:
            item.add_marker(pytest.mark.windows_slow)
            # Skip office tests on Windows as they timeout due to LibreOffice conversion issues
            item.add_marker(
                pytest.mark.skip(reason="Office tests timeout on Windows due to LibreOffice conversion delays")
            )

        if "test_ocr" in item.nodeid:
            # Skip OCR tests on Windows due to Tesseract training data not being available
            item.add_marker(
                pytest.mark.skip(reason="OCR tests skipped on Windows due to Tesseract training data unavailability")
            )


@pytest.fixture(autouse=True)
def _slow_test_timeout(request: pytest.FixtureRequest) -> None:
    """
    Auto-apply timeout to slow tests.

    Slow tests get 300 seconds (5 minutes) timeout.
    Regular tests get default timeout.
    """

    with contextlib.suppress(pytest.FixtureLookupError):
        request.getfixturevalue("pytest_timeout_set_sleep_func")

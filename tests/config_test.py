"""Tests for configuration and concurrency control."""

from __future__ import annotations

import multiprocessing
from typing import TYPE_CHECKING

import pytest

from kreuzberg.config import Config, default_config
from kreuzberg.extraction import extract_file

if TYPE_CHECKING:
    from pathlib import Path


def test_config_validation() -> None:
    """Test validation of Config."""

    Config()
    Config(max_concurrent_ocr=1)
    Config(max_concurrent_ocr=10)

    with pytest.raises(ValueError, match="max_concurrent_ocr must be at least 1"):
        Config(max_concurrent_ocr=0)

    with pytest.raises(ValueError, match="max_concurrent_ocr must be at least 1"):
        Config(max_concurrent_ocr=-1)


def test_config_concurrent_limit() -> None:
    """Test concurrent_limit calculation."""

    config = Config()
    expected_default = max(multiprocessing.cpu_count() // 2, 1)
    assert config.concurrent_limit == expected_default

    config = Config(max_concurrent_ocr=3)
    assert config.concurrent_limit == 3


async def test_tesseract_concurrency_control(ocr_image: Path) -> None:
    """Test that concurrency control works with Tesseract."""

    config1 = Config(max_concurrent_ocr=1)
    config2 = Config(max_concurrent_ocr=2)

    results1 = []
    results2 = []

    for _ in range(3):
        result = await extract_file(ocr_image, config=config1)
        results1.append(result.content)

    for _ in range(3):
        result = await extract_file(ocr_image, config=config2)
        results2.append(result.content)

    assert all(r == results1[0] for r in results1)
    assert all(r == results2[0] for r in results2)
    assert results1[0] == results2[0]


async def test_default_config_behavior(ocr_image: Path) -> None:
    """Test that default config is used when none is provided."""

    result1 = await extract_file(ocr_image, config=default_config)
    result2 = await extract_file(ocr_image)
    assert result1.content == result2.content

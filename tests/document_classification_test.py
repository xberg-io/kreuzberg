"""Tests for document classification functionality."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING
from unittest.mock import Mock, patch

import pandas as pd
import pytest

from kreuzberg._document_classification import (
    DOCUMENT_CLASSIFIERS,
    auto_detect_document_type,
    classify_document,
    classify_document_from_layout,
)
from kreuzberg._types import ExtractionConfig, ExtractionResult

if TYPE_CHECKING:
    from pathlib import Path


@pytest.mark.anyio
@pytest.mark.parametrize(
    ("doc_type", "file_name"),
    [
        ("invoice", "invoice_test.txt"),
        ("receipt", "receipt_test.txt"),
        ("contract", "contract_test.txt"),
        ("report", "report_test.txt"),
        ("form", "form_test.txt"),
    ],
)
async def test_extract_file_with_all_document_types(doc_type: str, file_name: str, test_files_path: Path) -> None:
    """Test that document classification works for all document types."""
    from kreuzberg import extract_file

    test_file = test_files_path / file_name
    config = ExtractionConfig(auto_detect_document_type=True)
    result = await extract_file(test_file, config=config)
    assert result.document_type == doc_type
    assert result.document_type_confidence is not None
    assert result.document_type_confidence > 0.5


def test_classify_document_low_confidence() -> None:
    """Test that no document type is returned for low confidence."""
    result = ExtractionResult(
        content="Some random text that doesn't match any patterns",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig(document_type_confidence_threshold=0.9)

    doc_type, confidence = classify_document(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_high_confidence() -> None:
    """Test that document type is returned for high confidence."""
    result = ExtractionResult(
        content="INVOICE #12345 Total: $100.00 Due Date: 01/01/2024",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig(document_type_confidence_threshold=0.1)

    doc_type, confidence = classify_document(result, config)

    assert doc_type == "invoice"
    assert confidence is not None
    assert confidence > 0.1


def test_document_classifiers_available() -> None:
    """Test that all document classifiers are available."""
    expected_types = {"invoice", "receipt", "contract", "report", "form"}
    assert set(DOCUMENT_CLASSIFIERS.keys()) == expected_types

    for classifier in DOCUMENT_CLASSIFIERS.values():
        assert hasattr(classifier, "patterns")
        assert hasattr(classifier, "keywords")
        assert hasattr(classifier, "exclusions")


def test_document_classifiers_patterns() -> None:
    """Test that document classifiers have valid patterns."""
    for classifier in DOCUMENT_CLASSIFIERS.values():
        assert isinstance(classifier.patterns, list)
        assert len(classifier.patterns) > 0

        for pattern in classifier.patterns:
            assert isinstance(pattern, str)
            assert len(pattern) > 0


def test_document_classifiers_keywords() -> None:
    """Test that document classifiers have valid keywords."""
    for classifier in DOCUMENT_CLASSIFIERS.values():
        assert isinstance(classifier.keywords, list)
        assert len(classifier.keywords) > 0

        for keyword in classifier.keywords:
            assert isinstance(keyword, str)
            assert len(keyword) > 0


def test_classify_document_with_metadata() -> None:
    """Test classification with metadata content."""
    result = ExtractionResult(
        content="Regular content",
        mime_type="text/plain",
        metadata={"title": "Invoice #12345", "subject": "Payment Due"},
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document(result, config)

    assert doc_type == "invoice"
    assert confidence is not None


def test_classify_document_disabled() -> None:
    """Test classification when disabled in config."""
    result = ExtractionResult(
        content="INVOICE #12345 Total: $100.00",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig(auto_detect_document_type=False)

    doc_type, confidence = classify_document(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_empty_content() -> None:
    """Test classification with empty content."""
    result = ExtractionResult(
        content="",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_with_exclusions() -> None:
    """Test that exclusion patterns work correctly."""
    # Contract classifier has exclusions for invoice/receipt terms
    result = ExtractionResult(
        content="CONTRACT AGREEMENT INVOICE #12345 Total: $100.00",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document(result, config)

    # Should classify as invoice due to strong invoice indicators
    assert doc_type == "invoice"


def test_classify_document_from_layout_basic() -> None:
    """Test basic layout-based classification."""
    layout_df = pd.DataFrame(
        {
            "text": ["INVOICE", "#12345", "Total:", "$100.00"],
            "top": [10, 30, 100, 120],
            "height": [20, 15, 15, 15],
        }
    )

    result = ExtractionResult(
        content="INVOICE #12345 Total: $100.00",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type == "invoice"
    assert confidence is not None
    assert confidence > 0.5


def test_classify_document_from_layout_no_layout() -> None:
    """Test layout classification when no layout data is available."""
    result = ExtractionResult(
        content="INVOICE #12345",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_from_layout_empty_layout() -> None:
    """Test layout classification with empty layout."""
    layout_df = pd.DataFrame()

    result = ExtractionResult(
        content="INVOICE #12345",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_from_layout_missing_columns() -> None:
    """Test layout classification with missing required columns."""
    layout_df = pd.DataFrame({"text": ["Test"], "missing_columns": [1]})

    result = ExtractionResult(
        content="Test content",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_from_layout_no_pattern_matches() -> None:
    """Test layout classification with no pattern matches."""
    layout_df = pd.DataFrame(
        {
            "text": ["Generic text", "No patterns here", "Just regular content"],
            "top": [10, 50, 100],
            "height": [20, 20, 20],
        }
    )

    result = ExtractionResult(
        content="Generic text No patterns here Just regular content",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type is None
    assert confidence is None


def test_classify_document_from_layout_header_patterns() -> None:
    """Test layout classification focusing on header patterns."""
    # Large header text should boost confidence
    layout_df = pd.DataFrame(
        {
            "text": ["INVOICE", "Company Name", "Item description", "Total: $100"],
            "top": [10, 40, 200, 250],
            "height": [30, 20, 15, 15],  # INVOICE has larger height (header)
        }
    )

    result = ExtractionResult(
        content="INVOICE Company Name Item description Total: $100",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type == "invoice"
    assert confidence is not None
    # Header boost should increase confidence
    assert confidence > 0.6


def test_classify_document_from_layout_position_scoring() -> None:
    """Test that layout position affects scoring."""
    # Same content but different positions
    layout_df = pd.DataFrame(
        {
            "text": ["receipt", "store info", "items", "total"],
            "top": [5, 30, 200, 300],  # "receipt" at very top
            "height": [15, 15, 15, 15],
        }
    )

    result = ExtractionResult(
        content="receipt store info items total",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = classify_document_from_layout(result, config)

    assert doc_type == "receipt"
    assert confidence is not None


def test_auto_detect_document_type_from_content() -> None:
    """Test auto-detection prioritizing content-based classification."""
    result = ExtractionResult(
        content="INVOICE #12345 Amount Due: $500.00 Payment Terms: Net 30",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig()

    doc_type, confidence = auto_detect_document_type(result, config)

    assert doc_type == "invoice"
    assert confidence is not None
    assert confidence > 0.5


def test_auto_detect_document_type_from_layout() -> None:
    """Test auto-detection falling back to layout when content is weak."""
    layout_df = pd.DataFrame(
        {
            "text": ["RECEIPT", "Store: ABC Shop", "Item: Coffee", "Total: $5.00"],
            "top": [10, 30, 100, 120],
            "height": [25, 15, 15, 15],
        }
    )

    result = ExtractionResult(
        content="Generic text without strong patterns",
        mime_type="text/plain",
        metadata={},
        layout=layout_df,
    )
    config = ExtractionConfig()

    doc_type, confidence = auto_detect_document_type(result, config)

    assert doc_type == "receipt"
    assert confidence is not None


def test_auto_detect_document_type_disabled() -> None:
    """Test auto-detection when disabled."""
    result = ExtractionResult(
        content="INVOICE #12345",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig(auto_detect_document_type=False)

    doc_type, confidence = auto_detect_document_type(result, config)

    assert doc_type is None
    assert confidence is None


def test_auto_detect_document_type_no_matches() -> None:
    """Test auto-detection when no classification matches."""
    result = ExtractionResult(
        content="Random text with no document indicators",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig()

    doc_type, confidence = auto_detect_document_type(result, config)

    assert doc_type is None
    assert confidence is None


def test_auto_detect_document_type_confidence_threshold() -> None:
    """Test auto-detection respects confidence threshold."""
    result = ExtractionResult(
        content="Maybe an invoice but not very clear",
        mime_type="text/plain",
        metadata={},
    )
    config = ExtractionConfig(document_type_confidence_threshold=0.9)

    doc_type, confidence = auto_detect_document_type(result, config)

    # Should return None if confidence is below threshold
    assert doc_type is None
    assert confidence is None


@pytest.mark.anyio
async def test_document_classification_integration_invoice(test_files_path: Path) -> None:
    """Test end-to-end document classification for invoice."""
    from kreuzberg import extract_file

    invoice_file = test_files_path / "invoice_test.txt"
    config = ExtractionConfig(auto_detect_document_type=True)

    result = await extract_file(invoice_file, config=config)

    assert result.document_type == "invoice"
    assert result.document_type_confidence is not None
    assert result.document_type_confidence > 0.5


@pytest.mark.anyio
async def test_document_classification_integration_receipt(test_files_path: Path) -> None:
    """Test end-to-end document classification for receipt."""
    from kreuzberg import extract_file

    receipt_file = test_files_path / "receipt_test.txt"
    config = ExtractionConfig(auto_detect_document_type=True)

    result = await extract_file(receipt_file, config=config)

    assert result.document_type == "receipt"
    assert result.document_type_confidence is not None
    assert result.document_type_confidence > 0.5


@pytest.mark.anyio
async def test_document_classification_integration_contract(test_files_path: Path) -> None:
    """Test end-to-end document classification for contract."""
    from kreuzberg import extract_file

    contract_file = test_files_path / "contract_test.txt"
    config = ExtractionConfig(auto_detect_document_type=True)

    result = await extract_file(contract_file, config=config)

    assert result.document_type == "contract"
    assert result.document_type_confidence is not None
    assert result.document_type_confidence > 0.5


@pytest.mark.anyio
async def test_document_classification_integration_report(test_files_path: Path) -> None:
    """Test end-to-end document classification for report."""
    from kreuzberg import extract_file

    report_file = test_files_path / "report_test.txt"
    config = ExtractionConfig(auto_detect_document_type=True)

    result = await extract_file(report_file, config=config)

    assert result.document_type == "report"
    assert result.document_type_confidence is not None
    assert result.document_type_confidence > 0.5


@pytest.mark.anyio
async def test_document_classification_integration_form(test_files_path: Path) -> None:
    """Test end-to-end document classification for form."""
    from kreuzberg import extract_file

    form_file = test_files_path / "form_test.txt"
    config = ExtractionConfig(auto_detect_document_type=True)

    result = await extract_file(form_file, config=config)

    assert result.document_type == "form"
    assert result.document_type_confidence is not None
    assert result.document_type_confidence > 0.5


@pytest.mark.anyio
async def test_document_classification_integration_disabled(test_files_path: Path) -> None:
    """Test that classification can be disabled."""
    from kreuzberg import extract_file

    invoice_file = test_files_path / "invoice_test.txt"
    config = ExtractionConfig(auto_detect_document_type=False)

    result = await extract_file(invoice_file, config=config)

    assert result.document_type is None
    assert result.document_type_confidence is None
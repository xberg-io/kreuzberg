"""Tests for ExtractionResult.pages attribute type hints and runtime behavior."""

from pathlib import Path

from kreuzberg import (
    ExtractionResult,
)


def test_extraction_result_has_pages_attribute() -> None:
    """Verify that ExtractionResult class has pages attribute."""
    result_attrs = dir(ExtractionResult)
    assert "pages" in result_attrs, "ExtractionResult missing 'pages' attribute"


def test_pages_field_type_annotation() -> None:
    """Verify ExtractionResult.pages has correct type annotation in stub file."""
    stub_file = Path(__file__).parent.parent / "kreuzberg" / "_internal_bindings.pyi"
    assert stub_file.exists(), f"Stub file not found: {stub_file}"

    stub_content = stub_file.read_text()
    assert "pages:" in stub_content, "pages field missing from type stub"
    assert "list[PageContent] | None" in stub_content, "pages type should be 'list[PageContent] | None' in stub"

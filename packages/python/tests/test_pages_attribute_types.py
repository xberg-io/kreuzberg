"""Tests for ExtractionResult.pages attribute type hints and runtime behavior."""

from pathlib import Path

import pytest

from kreuzberg import (
    ExtractionConfig,
    ExtractionResult,
    PageConfig,
    extract_file,
    extract_file_sync,
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


def test_pages_is_none_by_default(tmp_path: Path) -> None:
    """When page extraction is not enabled, pages should be None."""
    test_file = tmp_path / "test.txt"
    test_file.write_text("Hello world\n" * 50)

    config = ExtractionConfig()
    result = extract_file_sync(str(test_file), config=config)

    assert result.pages is None, "pages should be None when extract_pages=False"


def test_pages_attribute_with_page_extraction_enabled(tmp_path: Path) -> None:
    """When page extraction is enabled, pages may be None or populated."""
    test_file = tmp_path / "test.txt"
    test_file.write_text("Content\n" * 100)

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(test_file), config=config)

    pages = result.pages
    assert pages is None or isinstance(pages, list), f"pages should be None or list, got {type(pages)}"


def test_pages_with_pdf_single_page(test_documents: Path) -> None:
    """Extract pages from a single-page PDF."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    assert result.pages is not None, "pages should be populated for PDF with extract_pages=True"
    assert isinstance(result.pages, list), "pages should be a list"
    assert len(result.pages) > 0, "pages should not be empty for single-page PDF"

    for page in result.pages:
        assert isinstance(page, dict), "page should be a dict"
        assert "page_number" in page, "page missing page_number"
        assert "content" in page, "page missing content"
        assert "tables" in page, "page missing tables"
        assert "images" in page, "page missing images"
        assert isinstance(page["page_number"], int), "page_number should be int"
        assert isinstance(page["content"], str), "content should be string"
        assert isinstance(page["tables"], list), "tables should be list"
        assert isinstance(page["images"], list), "images should be list"
        is_blank_val = page.get("is_blank")
        assert is_blank_val is None or isinstance(is_blank_val, bool), "is_blank should be bool or None"


def test_pages_with_pdf_multipage(test_documents: Path) -> None:
    """Extract pages from a multi-page PDF and verify structure."""
    pdf_path = test_documents / "pdf" / "multi_page.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    assert result.pages is not None, "pages should be populated for multi-page PDF"
    assert isinstance(result.pages, list), "pages should be a list"
    assert len(result.pages) > 1, "should extract multiple pages"

    page_numbers = []
    for page in result.pages:
        assert isinstance(page, dict), "page should be a dict"
        assert "page_number" in page, "page missing page_number"
        page_numbers.append(page["page_number"])

    assert page_numbers == sorted(page_numbers), "page_numbers should be in order"


def test_pages_none_without_config(test_documents: Path) -> None:
    """Without PageConfig, pages should be None."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig()
    result = extract_file_sync(str(pdf_path), config=config)

    assert result.pages is None, "pages should be None without PageConfig"


def test_pages_empty_config(test_documents: Path) -> None:
    """With extract_pages=False (default), pages should be None."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=False))
    result = extract_file_sync(str(pdf_path), config=config)

    assert result.pages is None, "pages should be None when extract_pages=False"


def test_pages_iteration_safe_when_none(tmp_path: Path) -> None:
    """Iterating over result.pages when None should not raise TypeError."""
    test_file = tmp_path / "test.txt"
    test_file.write_text("Content\n" * 50)

    config = ExtractionConfig()
    result = extract_file_sync(str(test_file), config=config)

    if result.pages:
        for page in result.pages:
            assert isinstance(page, dict), f"page should be dict, got {type(page)}"


def test_pages_iteration_safe_when_populated(test_documents: Path) -> None:
    """Iterating over populated pages should work without errors."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    if result.pages:
        count = 0
        for page in result.pages:
            count += 1
            assert isinstance(page, dict), "page should be dict"
        assert count > 0, "should have iterated over pages"


def test_page_content_structure(test_documents: Path) -> None:
    """Verify all PageContent fields are accessible and have correct types."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    assert result.pages is not None
    assert len(result.pages) > 0
    page = result.pages[0]

    assert page["page_number"] >= 1, "page_number should be >= 1"
    assert isinstance(page["content"], str), "content should be string"
    assert len(page["content"]) > 0, "content should not be empty"
    assert isinstance(page["tables"], list), "tables should be list"
    assert isinstance(page["images"], list), "images should be list"
    is_blank_val = page.get("is_blank")
    assert is_blank_val is None or isinstance(is_blank_val, bool), "is_blank should be bool or None"


def test_pages_with_sync_extraction(test_documents: Path) -> None:
    """Verify pages work correctly with sync extraction."""
    pdf_path = test_documents / "pdf" / "multi_page.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    assert result.pages is not None
    assert isinstance(result.pages, list)
    assert len(result.pages) > 0


@pytest.mark.asyncio
async def test_pages_with_async_extraction(test_documents: Path) -> None:
    """Verify pages work correctly with async extraction."""
    pdf_path = test_documents / "pdf" / "multi_page.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = await extract_file(str(pdf_path), config=config)

    assert result.pages is not None
    assert isinstance(result.pages, list)
    assert len(result.pages) > 0


def test_pages_type_hint_with_mypy(test_documents: Path) -> None:
    """Verify type hints are properly recognized (pseudo-test for mypy)."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    pages = result.pages
    assert pages is None or isinstance(pages, list)

    if pages is not None:
        for page in pages:
            page_num: int = page["page_number"]
            content: str = page["content"]
            tables = page["tables"]
            images = page["images"]
            is_blank: bool | None = page.get("is_blank")
            assert isinstance(page_num, int)
            assert isinstance(content, str)
            assert isinstance(tables, list)
            assert isinstance(images, list)
            assert is_blank is None or isinstance(is_blank, bool)


def test_pages_edge_case_empty_pages(tmp_path: Path) -> None:
    """Edge case: document that produces empty pages list (if applicable)."""
    test_file = tmp_path / "blank.txt"
    test_file.write_text("")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(test_file), config=config)

    pages = result.pages
    assert pages is None or isinstance(pages, list)


def test_pages_accessible_after_extraction(test_documents: Path) -> None:
    """Verify pages attribute is accessible after extraction completes."""
    pdf_path = test_documents / "pdf" / "fake_memo.pdf"
    if not pdf_path.exists():
        pytest.skip(f"Test file not found: {pdf_path}")

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(pdf_path), config=config)

    assert hasattr(result, "pages"), "result should have pages attribute"
    assert result.pages is not None
    assert len(result.pages) > 0


def test_no_type_error_when_iterating_pages(tmp_path: Path) -> None:
    """Iterating over result.pages should not raise TypeError."""
    test_file = tmp_path / "test.txt"
    test_file.write_text("Content\n" * 100)

    config = ExtractionConfig(pages=PageConfig(extract_pages=True))
    result = extract_file_sync(str(test_file), config=config)

    if result.pages:
        for page in result.pages:
            assert isinstance(page, dict), f"page should be dict, got {type(page)}"

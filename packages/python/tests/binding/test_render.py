# Hand-written binding-specific edge case tests for PDF rendering.
# Happy-path render tests are auto-generated from fixtures in e2e/.
# These tests cover error handling, validation, and lifecycle patterns
# that vary per language and can't be generated uniformly.

from __future__ import annotations

from pathlib import Path

import pytest

from kreuzberg import PdfPageIterator, render_pdf_page

REPO_ROOT = Path(__file__).parent.parent.parent.parent.parent
TEST_PDF = REPO_ROOT / "test_documents" / "pdf" / "tiny.pdf"


def test_rendering_functions_exist() -> None:
    assert callable(render_pdf_page)
    assert callable(PdfPageIterator)


def test_render_pdf_page_nonexistent_file() -> None:
    with pytest.raises((OSError, RuntimeError), match=r"(No such file|cannot find the path|does not exist)"):
        render_pdf_page("/nonexistent/path/to/document.pdf", 0)


def test_render_pdf_page_out_of_bounds() -> None:
    if not TEST_PDF.exists():
        pytest.skip(f"Test PDF not found at {TEST_PDF}")
    with pytest.raises(RuntimeError, match="not found"):
        render_pdf_page(str(TEST_PDF), 9999)


def test_render_pdf_page_negative_index() -> None:
    if not TEST_PDF.exists():
        pytest.skip(f"Test PDF not found at {TEST_PDF}")
    with pytest.raises(OverflowError):
        render_pdf_page(str(TEST_PDF), -1)


def test_iterator_nonexistent_file() -> None:
    with pytest.raises((OSError, RuntimeError), match=r"(No such file|cannot find the path|does not exist)"):
        with PdfPageIterator("/nonexistent/path/to/document.pdf") as _it:
            pass


def test_iterator_cleanup() -> None:
    if not TEST_PDF.exists():
        pytest.skip(f"Test PDF not found at {TEST_PDF}")
    with PdfPageIterator(str(TEST_PDF)) as it:
        first = next(iter(it))
        assert isinstance(first, tuple)
        assert len(first) == 2
    # Context manager exited — safe to use again or discard


def test_iterator_early_termination() -> None:
    if not TEST_PDF.exists():
        pytest.skip(f"Test PDF not found at {TEST_PDF}")
    with PdfPageIterator(str(TEST_PDF)) as it:
        for page_index, png in it:
            assert isinstance(page_index, int)
            assert isinstance(png, bytes)
            break  # Stop after first page


def test_render_pdf_page_empty_path() -> None:
    with pytest.raises((OSError, RuntimeError), match=r"(No such file|cannot find the path|does not exist)"):
        render_pdf_page("", 0)

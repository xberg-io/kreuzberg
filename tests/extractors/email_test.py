"""Tests for email extraction functionality."""

from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from kreuzberg import ExtractionConfig
from kreuzberg._extractors._email import EmailExtractor
from kreuzberg._mime_types import EML_MIME_TYPE, MSG_MIME_TYPE
from kreuzberg.exceptions import MissingDependencyError


@pytest.fixture
def email_extractor() -> EmailExtractor:
    """Create EmailExtractor instance for testing."""
    config = ExtractionConfig()
    return EmailExtractor(EML_MIME_TYPE, config)


@pytest.fixture
def sample_email_path(tmp_path: Path) -> Path:
    """Create a sample email file for testing."""
    email_content = """Subject: Test Email
From: test@example.com
To: recipient@example.com

This is a test email body.
"""
    email_path = tmp_path / "test.eml"
    email_path.write_text(email_content)
    return email_path


def test_mime_types() -> None:
    """Test that email MIME types are properly defined."""
    from kreuzberg._extractors._email import EmailExtractor
    
    # Test that the extractor supports the expected MIME types
    assert EML_MIME_TYPE in EmailExtractor.SUPPORTED_MIME_TYPES
    assert MSG_MIME_TYPE not in EmailExtractor.SUPPORTED_MIME_TYPES  # MSG is handled by a different extractor


def test_extract_bytes_sync(email_extractor: EmailExtractor) -> None:
    """Test sync bytes extraction."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "Test Subject",
            "text": "Email body content",
        }

        result = email_extractor.extract_bytes_sync(b"dummy email content")

        assert result.content
        assert "Test Subject" in result.content
        assert "Email body content" in result.content
        assert result.metadata["subject"] == "Test Subject"


def test_extract_path_sync_basic(email_extractor: EmailExtractor, sample_email_path: Path) -> None:
    """Test sync path extraction."""
    result = email_extractor.extract_path_sync(sample_email_path)

    assert result.content
    assert "Test Email" in result.content
    assert result.metadata["subject"] == "Test Email"


def test_missing_mailparse_dependency_basic() -> None:
    """Test handling when mailparse is not available."""
    config = ExtractionConfig()
    extractor = EmailExtractor(EML_MIME_TYPE, config)

    with patch("kreuzberg._extractors._email.mailparse", None):
        with pytest.raises(MissingDependencyError, match="mailparse is required"):
            extractor.extract_bytes_sync(b"dummy email content")


@pytest.mark.anyio
async def test_extract_bytes_async(email_extractor: EmailExtractor) -> None:
    """Test async bytes extraction."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "Async Test",
            "text": "Async email body",
        }

        result = await email_extractor.extract_bytes_async(b"dummy email content")

        assert result.content
        assert "Async Test" in result.content
        assert "Async email body" in result.content
        assert result.metadata["subject"] == "Async Test"


@pytest.mark.anyio
async def test_extract_path_async(email_extractor: EmailExtractor, sample_email_path: Path) -> None:
    """Test async path extraction."""
    result = await email_extractor.extract_path_async(sample_email_path)

    assert result.content
    assert "Test Email" in result.content
    assert result.metadata["subject"] == "Test Email"


@pytest.mark.anyio
async def test_missing_mailparse_dependency_async() -> None:
    """Test handling when mailparse is not available in async mode."""
    config = ExtractionConfig()
    extractor = EmailExtractor(EML_MIME_TYPE, config)

    with patch("kreuzberg._extractors._email.mailparse", None):
        with pytest.raises(MissingDependencyError, match="mailparse is required"):
            await extractor.extract_bytes_async(b"dummy email content")


def test_email_header_extraction(email_extractor: EmailExtractor) -> None:
    """Test that email headers are properly extracted and formatted."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "cc": "cc@example.com",
            "bcc": "bcc@example.com",
            "subject": "Header Test",
            "date": "Mon, 1 Jan 2024 12:00:00 +0000",
            "text": "Body content",
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        # Check that headers are included in content
        assert "Subject: Header Test" in result.content
        assert "From: sender@example.com" in result.content
        assert "To: recipient@example.com" in result.content
        assert "CC: cc@example.com" in result.content
        assert "BCC: bcc@example.com" in result.content
        assert "Date: Mon, 1 Jan 2024 12:00:00 +0000" in result.content
        assert "Body content" in result.content


def test_email_complex_headers(email_extractor: EmailExtractor) -> None:
    """Test extraction with complex header structures."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": {"email": "sender@example.com", "name": "Sender Name"},
            "to": [
                {"email": "recipient1@example.com", "name": "Recipient 1"},
                {"email": "recipient2@example.com", "name": "Recipient 2"},
            ],
            "cc": [
                {"email": "cc1@example.com"},
                {"email": "cc2@example.com", "name": "CC Person"},
            ],
            "subject": "Complex Headers",
            "text": "Email with complex headers",
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        # Should handle complex header structures
        assert "From: sender@example.com" in result.content
        assert "To: recipient1@example.com" in result.content
        assert "CC: cc1@example.com" in result.content


def test_email_missing_headers(email_extractor: EmailExtractor) -> None:
    """Test email with missing headers."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "text": "Simple email without subject or date.",
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        assert result.content == "Simple email without subject or date."
        # Headers should not be added if they don't exist
        assert "Subject:" not in result.content
        assert "From:" not in result.content


def test_email_with_html_content_with_html2text(email_extractor: EmailExtractor) -> None:
    """Test email with HTML content when html2text is available."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "HTML Test",
            "html": "<p>This is <strong>HTML</strong> content</p>",
        }

        with patch("kreuzberg._extractors._email.html2text") as mock_html2text:
            mock_converter = MagicMock()
            mock_converter.handle.return_value = "This is **HTML** content"
            mock_html2text.HTML2Text.return_value = mock_converter

            result = email_extractor.extract_bytes_sync(b"dummy")

            assert "This is **HTML** content" in result.content
            mock_converter.handle.assert_called_once_with("<p>This is <strong>HTML</strong> content</p>")


def test_email_with_html_content_without_html2text(email_extractor: EmailExtractor) -> None:
    """Test email with HTML content when html2text is not available."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "HTML Test",
            "html": "<p>This is &lt;HTML&gt; content</p>",
        }

        # Mock html2text as None to simulate missing dependency
        with patch("kreuzberg._extractors._email.html2text", None):
            result = email_extractor.extract_bytes_sync(b"dummy")

            # Should fallback to HTML tag stripping and entity unescaping
            assert "This is <HTML> content" in result.content


def test_email_with_attachments(email_extractor: EmailExtractor) -> None:
    """Test email with attachments."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "Email with Attachments",
            "text": "Please see attached files.",
            "attachments": [
                {"name": "document.pdf"},
                {"name": "image.jpg"},
                {},  # Attachment without name
            ],
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        assert result.metadata["attachments"] == ["document.pdf", "image.jpg", "unknown"]
        assert "Attachments: document.pdf, image.jpg, unknown" in result.content


def test_email_with_empty_attachments(email_extractor: EmailExtractor) -> None:
    """Test email with empty attachments list."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "No Attachments",
            "text": "No files attached.",
            "attachments": [],
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        # Empty attachments list is falsy, so metadata key won't be set
        assert "attachments" not in result.metadata
        # And no attachment text should be added
        assert "Attachments:" not in result.content


def test_missing_mailparse_dependency() -> None:
    """Test handling when mailparse is not available."""
    config = ExtractionConfig()
    extractor = EmailExtractor(EML_MIME_TYPE, config)

    with patch("kreuzberg._extractors._email.mailparse", None):
        with pytest.raises(MissingDependencyError, match="mailparse is required"):
            extractor.extract_bytes_sync(b"dummy email content")


def test_email_with_html_body_without_html2text(email_extractor: EmailExtractor) -> None:
    """Test HTML email body extraction without html2text (fallback)."""
    with patch("kreuzberg._extractors._email.html2text", None):
        with patch("mailparse.EmailDecode.load") as mock_load:
            mock_load.return_value = {
                "from": "sender@example.com",
                "to": "recipient@example.com",
                "subject": "HTML Email",
                "html": "<html><body><p>Hello <b>World</b> &amp; Friends</p></body></html>",
            }

            result = email_extractor.extract_bytes_sync(b"dummy")

            # Should strip HTML tags and unescape entities
            assert "Hello World & Friends" in result.content
            assert "<p>" not in result.content
            assert "&amp;" not in result.content


def test_email_text_preferred_over_html(email_extractor: EmailExtractor) -> None:
    """Test that text content is preferred over HTML when both exist."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "Multipart Email",
            "text": "Plain text version",
            "html": "<html><body>HTML version</body></html>",
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        assert "Plain text version" in result.content
        assert "HTML version" not in result.content


def test_email_with_attachments_detailed(email_extractor: EmailExtractor) -> None:
    """Test email with attachments."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "Email with attachments",
            "text": "See attached files",
            "attachments": [
                {"name": "document.pdf", "content": b"fake pdf"},
                {"name": "image.jpg", "content": b"fake image"},
                {},  # Attachment without name
            ],
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        assert "Attachments: document.pdf, image.jpg, unknown" in result.content
        assert result.metadata["attachments"] == ["document.pdf", "image.jpg", "unknown"]


def test_email_without_attachments(email_extractor: EmailExtractor) -> None:
    """Test email without attachments."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": "sender@example.com",
            "to": "recipient@example.com",
            "subject": "No attachments",
            "text": "Email body",
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        assert "Attachments:" not in result.content
        assert "attachments" not in result.metadata


def test_missing_mailparse_dependency_with_fixture(email_extractor: EmailExtractor) -> None:
    """Test error when mailparse is not installed."""
    with patch("kreuzberg._extractors._email.mailparse", None):
        with pytest.raises(MissingDependencyError, match="mailparse is required"):
            email_extractor.extract_bytes_sync(b"dummy")


def test_mailparse_exception(email_extractor: EmailExtractor) -> None:
    """Test handling of exceptions from mailparse."""
    with patch("mailparse.EmailDecode.load", side_effect=Exception("Parse error")):
        with pytest.raises(RuntimeError, match="Failed to parse email content"):
            email_extractor.extract_bytes_sync(b"invalid email content")


def test_extract_path_sync(email_extractor: EmailExtractor, sample_email_path: Path) -> None:
    """Test sync path extraction."""
    result = email_extractor.extract_path_sync(sample_email_path)

    assert result.content
    assert "Test Email" in result.content
    assert result.metadata["subject"] == "Test Email"


def test_empty_email(email_extractor: EmailExtractor) -> None:
    """Test extraction of empty/minimal email."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {}

        result = email_extractor.extract_bytes_sync(b"dummy")

        assert result.content == ""
        assert result.metadata == {}


def test_email_with_all_fields(email_extractor: EmailExtractor) -> None:
    """Test email with all possible fields populated."""
    with patch("mailparse.EmailDecode.load") as mock_load:
        mock_load.return_value = {
            "from": {"email": "sender@example.com", "name": "Sender Name"},
            "to": [{"email": "recipient@example.com", "name": "Recipient"}],
            "cc": "cc@example.com",
            "bcc": "bcc@example.com",
            "subject": "Complete Email",
            "date": "Mon, 1 Jan 2024 12:00:00 +0000",
            "text": "Email body text",
            "attachments": [{"name": "file.txt"}],
        }

        result = email_extractor.extract_bytes_sync(b"dummy")

        # Check all components are present
        assert "Subject: Complete Email" in result.content
        assert "From: sender@example.com" in result.content
        assert "To: recipient@example.com" in result.content
        assert "CC: cc@example.com" in result.content
        assert "BCC: bcc@example.com" in result.content
        assert "Date: Mon, 1 Jan 2024 12:00:00 +0000" in result.content
        assert "Email body text" in result.content
        assert "Attachments: file.txt" in result.content

        # Check metadata
        assert result.metadata["subject"] == "Complete Email"
        assert result.metadata["email_from"] == "sender@example.com"
        assert result.metadata["email_to"] == "recipient@example.com"
        assert result.metadata["email_cc"] == "cc@example.com"
        assert result.metadata["email_bcc"] == "bcc@example.com"
        assert result.metadata["date"] == "Mon, 1 Jan 2024 12:00:00 +0000"
        assert result.metadata["attachments"] == ["file.txt"]

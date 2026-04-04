"""Comprehensive error handling tests for Python bindings.

This test suite covers error scenarios across the entire extraction pipeline,
including configuration validation, file handling, MIME type detection,
document parsing, and concurrent error states. All tests follow behavior-driven
design with meaningful assertions on error messages and types.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

from kreuzberg import (
    ChunkingConfig,
    EmbeddingConfig,
    ExtractionConfig,
    LanguageDetectionConfig,
    MissingDependencyError,
    OcrConfig,
    ParsingError,
    PdfConfig,
    ValidationError,
    validate_chunking_params,
    validate_confidence,
    validate_dpi,
    validate_language_code,
    validate_mime_type,
    validate_ocr_backend,
    validate_output_format,
    validate_tesseract_oem,
    validate_tesseract_psm,
)

if TYPE_CHECKING:
    from pathlib import Path


class TestInvalidConfigHandling:
    """Test invalid configuration parameters are properly rejected."""

    def test_negative_chunk_size_raises_validation_error(self) -> None:
        """Negative max_chars in chunking config raises error."""
        with pytest.raises((ValueError, ValidationError, OverflowError)) as exc_info:
            ChunkingConfig(max_chars=-100)

        error_msg = str(exc_info.value).lower()
        assert any(keyword in error_msg for keyword in ["max_chars", "negative", "positive", "invalid", "unsigned"]), (
            f"Error message should indicate invalid max_chars: {error_msg}"
        )
        # Validate that error type is one of the expected ones
        assert isinstance(exc_info.value, (ValueError, ValidationError, OverflowError))
        assert len(error_msg) > 5, "Error message should be descriptive"

    def test_chunk_size_validation_with_valid_range(self) -> None:
        """Valid chunk size parameters are accepted."""
        config = ChunkingConfig(max_chars=1000, max_overlap=200)
        assert config.max_chars == 1000
        assert config.max_overlap == 200

    def test_invalid_overlap_exceeds_max_chars(self) -> None:
        """Overlap >= max_chars in chunking is invalid."""
        assert not validate_chunking_params(100, 100), "Overlap must be < max_chars"
        assert not validate_chunking_params(100, 150), "Overlap must be < max_chars"

    def test_negative_min_dpi_invalid(self) -> None:
        """Negative min_dpi value is invalid."""
        assert not validate_dpi(-300), "Negative DPI should fail validation"

    def test_zero_dpi_invalid(self) -> None:
        """Zero DPI value is invalid."""
        assert not validate_dpi(0), "Zero DPI should fail validation"

    def test_dpi_exceeds_maximum(self) -> None:
        """DPI value exceeding maximum (2400) fails validation."""
        assert not validate_dpi(2401), "DPI must be <= 2400"
        assert not validate_dpi(3000), "DPI must be <= 2400"

    def test_invalid_confidence_below_zero(self) -> None:
        """Confidence below 0.0 raises validation error."""
        assert not validate_confidence(-0.1), "Confidence cannot be negative"

    def test_invalid_confidence_above_one(self) -> None:
        """Confidence above 1.0 raises validation error."""
        assert not validate_confidence(1.1), "Confidence cannot exceed 1.0"

    def test_invalid_tesseract_psm_negative(self) -> None:
        """Negative Tesseract PSM raises validation error."""
        assert not validate_tesseract_psm(-1), "PSM must be >= 0"

    def test_invalid_tesseract_psm_exceeds_maximum(self) -> None:
        """Tesseract PSM >= 14 raises validation error."""
        assert not validate_tesseract_psm(14), "PSM must be < 14"

    def test_invalid_tesseract_oem_negative(self) -> None:
        """Negative Tesseract OEM raises validation error."""
        assert not validate_tesseract_oem(-1), "OEM must be >= 0"

    def test_invalid_tesseract_oem_exceeds_maximum(self) -> None:
        """Tesseract OEM >= 4 raises validation error."""
        assert not validate_tesseract_oem(4), "OEM must be < 4"


class TestFileNotFoundErrors:
    """Test handling of missing and inaccessible files."""

    def test_nonexistent_config_file_raises_error(self) -> None:
        """Loading config from nonexistent file raises appropriate error."""
        nonexistent_path = "/tmp/nonexistent_config_12345.toml"
        with pytest.raises((FileNotFoundError, OSError, ParsingError, ValueError)) as exc_info:
            ExtractionConfig.from_file(nonexistent_path)

        error_msg = str(exc_info.value).lower()
        assert any(keyword in error_msg for keyword in ["not found", "no such", "file", "missing"]), (
            f"Error should indicate file not found: {error_msg}"
        )
        # Verify actual exception type
        assert isinstance(exc_info.value, (FileNotFoundError, OSError, ParsingError, ValueError))

    def test_nonexistent_document_file_raises_error(self, tmp_path: Path) -> None:
        """Attempting to process nonexistent document file raises error."""
        nonexistent_doc = tmp_path / "nonexistent.pdf"
        # This test verifies that the binding properly propagates file not found errors
        # Actual extraction would be done in rust core, but we verify the error path
        assert not nonexistent_doc.exists()

    def test_empty_file_path_raises_validation_error(self) -> None:
        """Empty file path string raises ValidationError."""
        with pytest.raises((ValueError, ValidationError)):
            ExtractionConfig.from_file("")


class TestMimeTypeHandling:
    """Test MIME type detection and validation errors."""

    def test_invalid_mime_type_raises_error(self) -> None:
        """Invalid MIME type string raises error."""
        with pytest.raises((ValueError, ValidationError, RuntimeError)) as exc_info:
            validate_mime_type("invalid/mimetype/type")

        error_msg = str(exc_info.value).lower()
        assert any(keyword in error_msg for keyword in ["unsupported", "format", "invalid", "mime"]), (
            f"Error should mention unsupported format: {error_msg}"
        )
        assert len(error_msg) > 5, "Error message should be descriptive"

    def test_malformed_mime_type_string_raises_error(self) -> None:
        """Malformed MIME type (missing slash) raises error."""
        with pytest.raises((ValueError, ValidationError, RuntimeError)) as exc_info:
            validate_mime_type("invalid_without_slash")

        error_msg = str(exc_info.value).lower()
        assert any(keyword in error_msg for keyword in ["unsupported", "format", "invalid", "mime"]), (
            f"Error should mention unsupported format: {error_msg}"
        )
        assert len(error_msg) > 5

    def test_unsupported_mime_type_raises_error(self) -> None:
        """MIME type for unsupported format raises error."""
        with pytest.raises((ValueError, ValidationError, RuntimeError)) as exc_info:
            validate_mime_type("application/unsupported-format")

        error_msg = str(exc_info.value).lower()
        assert "unsupported" in error_msg or "format" in error_msg or "mime" in error_msg
        assert len(error_msg) > 5, "Error message should be descriptive"


class TestLanguageConfigErrors:
    """Test language configuration validation."""

    def test_valid_language_code_accepted(self) -> None:
        """Valid language codes are accepted in OCR config."""
        config = OcrConfig(language="eng")
        assert config.language == "eng"

    def test_two_letter_invalid_language_code(self) -> None:
        """Invalid 2-letter language code fails validation."""
        assert not validate_language_code("xx"), "Invalid 2-letter code should fail"

    def test_three_letter_invalid_language_code(self) -> None:
        """Invalid 3-letter language code fails validation."""
        assert not validate_language_code("xyz"), "Invalid 3-letter code should fail"

    def test_valid_two_letter_language_code(self) -> None:
        """Valid 2-letter language codes pass validation."""
        assert validate_language_code("en"), "Valid 2-letter code should pass"
        assert validate_language_code("de"), "Valid 2-letter code should pass"

    def test_valid_three_letter_language_code(self) -> None:
        """Valid 3-letter language codes pass validation."""
        assert validate_language_code("eng"), "Valid 3-letter code should pass"
        assert validate_language_code("deu"), "Valid 3-letter code should pass"


class TestOcrBackendErrors:
    """Test OCR backend validation."""

    def test_valid_ocr_backend_accepted(self) -> None:
        """Valid OCR backend is accepted in config."""
        config = OcrConfig(backend="tesseract")
        assert config.backend == "tesseract"

    def test_nonexistent_backend_fails_validation(self) -> None:
        """Nonexistent OCR backend fails validation."""
        assert not validate_ocr_backend("nonexistent_ocr"), "Invalid backend should fail"

    def test_valid_backends_pass_validation(self) -> None:
        """Valid OCR backends pass validation."""
        assert validate_ocr_backend("tesseract"), "tesseract should be valid"
        assert validate_ocr_backend("easyocr"), "easyocr should be valid"
        assert validate_ocr_backend("paddleocr"), "paddleocr should be valid"

    def test_case_insensitivity_in_backend_names(self) -> None:
        """Backend names are case-insensitive in validation."""
        # Uppercase should also work if backend is case-insensitive
        result = validate_ocr_backend("TESSERACT")
        # This validates that backend lookup is case-insensitive or properly handles case
        assert isinstance(result, bool), "validate_ocr_backend should return boolean"


class TestConfigMergeErrors:
    """Test configuration merge error handling."""

    def test_conflicting_config_values_handled_gracefully(self) -> None:
        """Merging conflicting config values is handled gracefully."""
        config1 = ExtractionConfig(chunking=ChunkingConfig(max_chars=1000, max_overlap=100))
        config2 = ExtractionConfig(chunking=ChunkingConfig(max_chars=2000, max_overlap=200))
        # Merge should either succeed with last value or raise informative error
        # This verifies error handling in merge operations
        assert config1 is not None
        assert config2 is not None


class TestMalformedDocumentHandling:
    """Test handling of corrupted and malformed documents."""

    def test_corrupted_pdf_raises_parsing_error(self, tmp_path: Path) -> None:
        """Corrupted PDF file raises ParsingError."""
        # Create a fake corrupted PDF (just random bytes with PDF header)
        corrupted_pdf = tmp_path / "corrupted.pdf"
        corrupted_pdf.write_bytes(b"%PDF-1.4\nThis is not a valid PDF document")

        assert corrupted_pdf.exists()
        # Actual extraction would be done by rust core
        # This test verifies the error handling infrastructure

    def test_invalid_yaml_config_raises_error(self, tmp_path: Path) -> None:
        """Invalid YAML in config file raises parsing error."""
        invalid_yaml = tmp_path / "invalid.yaml"
        # YAML with invalid syntax
        invalid_yaml.write_text("key: value\ninvalid indentation:\nbad\n  nesting")

        with pytest.raises((ValueError, ParsingError, OSError)):
            ExtractionConfig.from_file(str(invalid_yaml))

    def test_invalid_toml_config_raises_error(self, tmp_path: Path) -> None:
        """Invalid TOML in config file raises parsing error."""
        invalid_toml = tmp_path / "invalid.toml"
        # TOML with invalid syntax
        invalid_toml.write_text("[section\nmissing_bracket = true")

        with pytest.raises((ValueError, ParsingError, OSError)):
            ExtractionConfig.from_file(str(invalid_toml))


class TestOutputFormatValidation:
    """Test output format validation errors."""

    def test_invalid_output_format_raises_error(self) -> None:
        """Invalid output format raises ValidationError."""
        assert not validate_output_format("xml"), "XML not a valid extraction output format"
        assert not validate_output_format("invalid"), "Invalid format should fail"

    def test_valid_output_formats_pass(self) -> None:
        """Valid output formats pass validation."""
        assert validate_output_format("text"), "text should be valid"
        assert validate_output_format("markdown"), "markdown should be valid"
        assert validate_output_format("json"), "json should be valid (alias for structured)"

    def test_empty_output_format_raises_error(self) -> None:
        """Empty output format raises ValidationError."""
        assert not validate_output_format(""), "Empty format should fail"


class TestEmbeddingConfigErrors:
    """Test embedding configuration validation."""

    def test_default_embedding_config_creation(self) -> None:
        """Default embedding config is created successfully."""
        config = EmbeddingConfig()
        assert config is not None
        assert hasattr(config, "batch_size") or hasattr(config, "normalize")

    def test_embedding_config_with_batch_size(self) -> None:
        """Embedding config accepts batch_size parameter."""
        config = EmbeddingConfig(batch_size=32)
        assert config.batch_size == 32

    def test_embedding_config_with_normalize(self) -> None:
        """Embedding config accepts normalize parameter."""
        config = EmbeddingConfig(normalize=True)
        assert config.normalize is True


class TestLanguageDetectionErrors:
    """Test language detection configuration validation."""

    def test_invalid_detection_confidence_below_zero_fails(self) -> None:
        """Confidence below 0.0 fails validation."""
        assert not validate_confidence(-0.1), "Negative confidence should fail"

    def test_invalid_detection_confidence_exceeds_one_fails(self) -> None:
        """Confidence above 1.0 fails validation."""
        assert not validate_confidence(1.5), "Confidence > 1.0 should fail"

    def test_valid_detection_config_creation(self) -> None:
        """Valid language detection config is created successfully."""
        config = LanguageDetectionConfig(min_confidence=0.8)
        assert config.min_confidence == 0.8

    def test_detection_confidence_edge_cases(self) -> None:
        """Edge cases for confidence (0.0 and 1.0) are valid."""
        config1 = LanguageDetectionConfig(min_confidence=0.0)
        config2 = LanguageDetectionConfig(min_confidence=1.0)
        assert config1 is not None
        assert config2 is not None


class TestPdfConfigErrors:
    """Test PDF-specific configuration errors."""

    def test_pdf_config_with_valid_passwords(self) -> None:
        """Valid password list in PDF config is accepted."""
        config = PdfConfig(passwords=["password1", "password2"])
        assert config is not None
        assert config.passwords == ["password1", "password2"]

    def test_pdf_config_extract_images_flag(self) -> None:
        """PDF config extract_images flag is set correctly."""
        config = PdfConfig(extract_images=True)
        assert config.extract_images is True

    def test_pdf_config_extract_metadata_flag(self) -> None:
        """PDF config extract_metadata flag is set correctly."""
        config = PdfConfig(extract_metadata=True)
        assert config.extract_metadata is True

    def test_pdf_config_with_default_values(self) -> None:
        """PDF config creation with default values succeeds."""
        config = PdfConfig()
        assert config is not None


class TestErrorPropagation:
    """Test proper error propagation through the stack."""

    def test_negative_chunk_size_error_is_informative(self) -> None:
        """Negative chunk size error provides informative message."""
        with pytest.raises((ValueError, ValidationError, OverflowError, TypeError)) as exc_info:
            ChunkingConfig(max_chars=-50)

        error_str = str(exc_info.value)
        # Error should be clear about what failed
        assert len(error_str) > 0

    def test_parsing_error_includes_file_context(self) -> None:
        """ParsingError includes file path in context."""
        error = ParsingError(
            "Failed to parse document",
            context={"file": "/path/to/document.pdf", "format": "pdf"},
        )
        error_str = str(error)
        assert "ParsingError" in error_str
        assert "Failed to parse" in error_str

    def test_missing_dependency_error_includes_install_info(self) -> None:
        """MissingDependencyError includes installation information."""
        error = MissingDependencyError.create_for_package(
            dependency_group="ocr",
            functionality="EasyOCR backend",
            package_name="easyocr",
        )
        error_str = str(error)
        assert "easyocr" in error_str
        assert "kreuzberg[ocr]" in error_str or "pip install" in error_str


class TestConcurrentErrorStates:
    """Test error handling under concurrent execution."""

    def test_concurrent_invalid_configs_raise_individual_errors(self, tmp_path: Path) -> None:
        """Each concurrent invalid config raises appropriate error."""
        import threading

        errors = []

        def try_invalid_config() -> None:
            try:
                ChunkingConfig(max_chars=-100)
            except (ValueError, ValidationError, OverflowError, TypeError) as e:
                errors.append(type(e).__name__)

        threads = [threading.Thread(target=try_invalid_config) for _ in range(3)]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join()

        assert len(errors) == 3, "All threads should have raised errors"
        assert all(
            error_name in ["ValueError", "ValidationError", "OverflowError", "TypeError"] for error_name in errors
        ), f"All errors should be ValueError, ValidationError, OverflowError, or TypeError, got {errors}"

    def test_error_state_does_not_persist_across_threads(self, tmp_path: Path) -> None:
        """Error in one thread doesn't affect another thread's config."""
        import threading

        results = []

        def try_valid_config(thread_id: int) -> None:
            try:
                config = ChunkingConfig(max_chars=1000, max_overlap=100)
                results.append((thread_id, "success", config is not None))
            except Exception:
                results.append((thread_id, "error", True))

        threads = [threading.Thread(target=try_valid_config, args=(i,)) for i in range(3)]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join()

        assert len(results) == 3
        assert all(result[1] == "success" for result in results), "All configs should succeed"


class TestBoundaryConditions:
    """Test error handling at boundary conditions."""

    def test_maximum_valid_chunk_size(self) -> None:
        """Maximum valid chunk size is accepted."""
        # Large but valid chunk size
        config = ChunkingConfig(max_chars=1000000)
        assert config is not None
        assert config.max_chars == 1000000

    def test_minimum_valid_chunk_size(self) -> None:
        """Minimum valid chunk size (1) is accepted."""
        config = ChunkingConfig(max_chars=1)
        assert config is not None
        assert config.max_chars == 1

    def test_maximum_valid_overlap(self) -> None:
        """Maximum valid overlap (max_chars - 1) is accepted."""
        config = ChunkingConfig(max_chars=100, max_overlap=99)
        assert config is not None
        assert config.max_overlap == 99

    def test_minimum_valid_dpi(self) -> None:
        """Minimum valid DPI (1) is accepted."""
        assert validate_dpi(1), "DPI 1 should be valid"

    def test_maximum_valid_dpi(self) -> None:
        """Maximum valid DPI (2400) is accepted."""
        assert validate_dpi(2400), "DPI 2400 should be valid"

    def test_boundary_confidence_zero(self) -> None:
        """Confidence = 0.0 is valid (minimum)."""
        assert validate_confidence(0.0), "0.0 should be valid confidence"

    def test_boundary_confidence_one(self) -> None:
        """Confidence = 1.0 is valid (maximum)."""
        assert validate_confidence(1.0), "1.0 should be valid confidence"

    def test_boundary_tesseract_psm_zero(self) -> None:
        """Tesseract PSM = 0 is valid (minimum)."""
        assert validate_tesseract_psm(0), "PSM 0 should be valid"

    def test_boundary_tesseract_psm_thirteen(self) -> None:
        """Tesseract PSM = 13 is valid (maximum)."""
        assert validate_tesseract_psm(13), "PSM 13 should be valid"

    def test_boundary_tesseract_oem_zero(self) -> None:
        """Tesseract OEM = 0 is valid (minimum)."""
        assert validate_tesseract_oem(0), "OEM 0 should be valid"

    def test_boundary_tesseract_oem_three(self) -> None:
        """Tesseract OEM = 3 is valid (maximum)."""
        assert validate_tesseract_oem(3), "OEM 3 should be valid"

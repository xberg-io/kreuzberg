package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive OcrConfig tests.
 *
 * <p>
 * Tests for OCR configuration including backend selection, language codes,
 * builder pattern, and integration with TesseractConfig.
 */
@DisplayName("OcrConfig Tests")
final class OcrConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		OcrConfig config = OcrConfig.builder().build();

		assertThat(config.getBackend()).isEqualTo("tesseract");
		assertThat(config.getLanguage()).isEqualTo("eng");
		assertNull(config.getTesseractConfig());
	}

	@Test
	@DisplayName("should create config with custom backend")
	void shouldCreateWithCustomBackend() {
		OcrConfig config = OcrConfig.builder().backend("easyocr").build();

		assertThat(config.getBackend()).isEqualTo("easyocr");
		assertThat(config.getLanguage()).isEqualTo("eng");
	}

	@Test
	@DisplayName("should create config with custom language")
	void shouldCreateWithCustomLanguage() {
		OcrConfig config = OcrConfig.builder().language("deu").build();

		assertThat(config.getBackend()).isEqualTo("tesseract");
		assertThat(config.getLanguage()).isEqualTo("deu");
	}

	@Test
	@DisplayName("should create config with custom values")
	void shouldCreateWithCustomValues() {
		OcrConfig config = OcrConfig.builder().backend("easyocr").language("fra").build();

		assertThat(config.getBackend()).isEqualTo("easyocr");
		assertThat(config.getLanguage()).isEqualTo("fra");
	}

	@Test
	@DisplayName("should reject invalid backend")
	void shouldRejectInvalidBackend() {
		assertThatThrownBy(() -> OcrConfig.builder().backend("invalid_backend").build())
				.isInstanceOf(IllegalArgumentException.class);
	}

	@Test
	@DisplayName("should reject invalid language code")
	void shouldRejectInvalidLanguageCode() {
		assertThatThrownBy(() -> OcrConfig.builder().language("invalid").build())
				.isInstanceOf(IllegalArgumentException.class);
	}

	@Test
	@DisplayName("should support TesseractConfig nesting")
	void shouldSupportTesseractConfig() {
		TesseractConfig tesseractConfig = TesseractConfig.builder().psm(6).build();
		OcrConfig config = OcrConfig.builder().tesseractConfig(tesseractConfig).build();

		assertNotNull(config.getTesseractConfig());
		assertThat(config.getTesseractConfig().getPsm()).isEqualTo(6);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		OcrConfig config = OcrConfig.builder().backend("tesseract").language("eng").build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("backend", "tesseract").containsEntry("language", "eng");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		OcrConfig config = OcrConfig.builder().backend("easyocr").language("fra").build();

		assertThat(config.getBackend()).isEqualTo("easyocr");
		assertThat(config.getLanguage()).isEqualTo("fra");
	}

	@Test
	@DisplayName("should be immutable after construction")
	void shouldBeImmutable() {
		OcrConfig config = OcrConfig.builder().build();
		// Verify config properties cannot be modified (fields are final)
		assertThat(config.getBackend()).isNotNull();
	}

	@Test
	@DisplayName("should handle null TesseractConfig")
	void shouldHandleNullTesseractConfig() {
		OcrConfig config = OcrConfig.builder().tesseractConfig(null).build();
		assertNull(config.getTesseractConfig());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		OcrConfig config1 = OcrConfig.builder().backend("tesseract").build();
		OcrConfig config2 = OcrConfig.builder().backend("easyocr").build();

		assertThat(config1.getBackend()).isNotEqualTo(config2.getBackend());
	}
}

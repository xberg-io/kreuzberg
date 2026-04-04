package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive TesseractConfig tests.
 *
 * <p>
 * Tests for Tesseract-specific OCR configuration including PSM values, table
 * detection, and character whitelisting.
 */
@DisplayName("TesseractConfig Tests")
final class TesseractConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		TesseractConfig config = TesseractConfig.builder().build();

		assertNull(config.getPsm());
		assertNull(config.getEnableTableDetection());
		assertNull(config.getTesseditCharWhitelist());
	}

	@Test
	@DisplayName("should set PSM value")
	void shouldSetPsmValue() {
		TesseractConfig config = TesseractConfig.builder().psm(6).build();

		assertThat(config.getPsm()).isEqualTo(6);
	}

	@Test
	@DisplayName("should set enable table detection")
	void shouldSetTableDetection() {
		TesseractConfig config = TesseractConfig.builder().enableTableDetection(true).build();

		assertThat(config.getEnableTableDetection()).isTrue();
	}

	@Test
	@DisplayName("should set tessedit char whitelist")
	void shouldSetCharWhitelist() {
		TesseractConfig config = TesseractConfig.builder().tesseditCharWhitelist("0123456789abcdefghijklmnopqrstuvwxyz")
				.build();

		assertThat(config.getTesseditCharWhitelist()).isEqualTo("0123456789abcdefghijklmnopqrstuvwxyz");
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		TesseractConfig config = TesseractConfig.builder().psm(3).enableTableDetection(true)
				.tesseditCharWhitelist("ABC123").build();

		assertThat(config.getPsm()).isEqualTo(3);
		assertThat(config.getEnableTableDetection()).isTrue();
		assertThat(config.getTesseditCharWhitelist()).isEqualTo("ABC123");
	}

	@Test
	@DisplayName("should validate PSM value")
	void shouldValidatePsmValue() {
		assertThatThrownBy(() -> TesseractConfig.builder().psm(99).build())
				.isInstanceOf(IllegalArgumentException.class);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		TesseractConfig config = TesseractConfig.builder().psm(6).enableTableDetection(true).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("psm", 6).containsEntry("enable_table_detection", true);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		TesseractConfig config = TesseractConfig.builder().psm(3).enableTableDetection(false).build();

		assertThat(config.getPsm()).isEqualTo(3);
		assertThat(config.getEnableTableDetection()).isFalse();
	}

	@Test
	@DisplayName("should handle null optional fields")
	void shouldHandleNullOptionalFields() {
		TesseractConfig config = TesseractConfig.builder().psm(null).build();

		assertNull(config.getPsm());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		TesseractConfig config1 = TesseractConfig.builder().psm(3).build();
		TesseractConfig config2 = TesseractConfig.builder().psm(6).build();

		assertThat(config1.getPsm()).isNotEqualTo(config2.getPsm());
	}

	@Test
	@DisplayName("should support table detection toggle")
	void shouldSupportTableDetectionToggle() {
		TesseractConfig configEnabled = TesseractConfig.builder().enableTableDetection(true).build();
		TesseractConfig configDisabled = TesseractConfig.builder().enableTableDetection(false).build();

		assertThat(configEnabled.getEnableTableDetection()).isTrue();
		assertThat(configDisabled.getEnableTableDetection()).isFalse();
	}
}

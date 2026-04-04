package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive LanguageDetectionConfig tests.
 *
 * <p>
 * Tests for language detection configuration including enable flag, confidence
 * threshold, and support for multiple language detection.
 */
@DisplayName("LanguageDetectionConfig Tests")
final class LanguageDetectionConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().build();

		assertThat(config.isEnabled()).isFalse();
		assertThat(config.getMinConfidence()).isEqualTo(0.5);
		assertThat(config.isDetectMultiple()).isFalse();
	}

	@Test
	@DisplayName("should enable language detection")
	void shouldEnableLanguageDetection() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().enabled(true).build();

		assertThat(config.isEnabled()).isTrue();
	}

	@Test
	@DisplayName("should set min confidence threshold")
	void shouldSetMinConfidence() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().minConfidence(0.7).build();

		assertThat(config.getMinConfidence()).isEqualTo(0.7);
	}

	@Test
	@DisplayName("should enable detect multiple")
	void shouldEnableDetectMultiple() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().detectMultiple(true).build();

		assertThat(config.isDetectMultiple()).isTrue();
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().enabled(true).minConfidence(0.8)
				.detectMultiple(true).build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getMinConfidence()).isEqualTo(0.8);
		assertThat(config.isDetectMultiple()).isTrue();
	}

	@Test
	@DisplayName("should validate confidence threshold range")
	void shouldValidateConfidenceThreshold() {
		LanguageDetectionConfig config1 = LanguageDetectionConfig.builder().minConfidence(0.0).build();
		LanguageDetectionConfig config2 = LanguageDetectionConfig.builder().minConfidence(1.0).build();

		assertThat(config1.getMinConfidence()).isGreaterThanOrEqualTo(0.0);
		assertThat(config2.getMinConfidence()).isLessThanOrEqualTo(1.0);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().enabled(true).minConfidence(0.6)
				.detectMultiple(true).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("enabled", true).containsEntry("min_confidence", 0.6)
				.containsEntry("detect_multiple", true);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().enabled(true).minConfidence(0.75)
				.detectMultiple(true).build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getMinConfidence()).isEqualTo(0.75);
		assertThat(config.isDetectMultiple()).isTrue();
	}

	@Test
	@DisplayName("should handle various confidence thresholds")
	void shouldHandleVariousThresholds() {
		LanguageDetectionConfig config1 = LanguageDetectionConfig.builder().minConfidence(0.1).build();
		LanguageDetectionConfig config2 = LanguageDetectionConfig.builder().minConfidence(0.5).build();
		LanguageDetectionConfig config3 = LanguageDetectionConfig.builder().minConfidence(0.9).build();

		assertThat(config1.getMinConfidence()).isLessThan(config2.getMinConfidence());
		assertThat(config2.getMinConfidence()).isLessThan(config3.getMinConfidence());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		LanguageDetectionConfig config1 = LanguageDetectionConfig.builder().enabled(true).build();
		LanguageDetectionConfig config2 = LanguageDetectionConfig.builder().enabled(false).build();

		assertThat(config1.isEnabled()).isNotEqualTo(config2.isEnabled());
	}

	@Test
	@DisplayName("should support combined settings")
	void shouldSupportCombinedSettings() {
		LanguageDetectionConfig config = LanguageDetectionConfig.builder().enabled(true).minConfidence(0.75)
				.detectMultiple(true).build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getMinConfidence()).isGreaterThan(0.5);
		assertThat(config.isDetectMultiple()).isTrue();
	}
}

package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive ImageExtractionConfig tests.
 *
 * <p>
 * Tests for image extraction configuration including DPI settings, dimension
 * limits, and auto-adjustment features.
 */
@DisplayName("ImageExtractionConfig Tests")
final class ImageExtractionConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().build();

		assertThat(config.isExtractImages()).isTrue();
		assertThat(config.getTargetDpi()).isEqualTo(300);
		assertThat(config.getMaxImageDimension()).isEqualTo(2000);
		assertThat(config.isAutoAdjustDpi()).isTrue();
		assertThat(config.getMinDpi()).isEqualTo(150);
		assertThat(config.getMaxDpi()).isEqualTo(600);
	}

	@Test
	@DisplayName("should disable image extraction")
	void shouldDisableImageExtraction() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().extractImages(false).build();

		assertThat(config.isExtractImages()).isFalse();
	}

	@Test
	@DisplayName("should set target DPI")
	void shouldSetTargetDpi() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().targetDpi(600).build();

		assertThat(config.getTargetDpi()).isEqualTo(600);
	}

	@Test
	@DisplayName("should set max image dimension")
	void shouldSetMaxImageDimension() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().maxImageDimension(4000).build();

		assertThat(config.getMaxImageDimension()).isEqualTo(4000);
	}

	@Test
	@DisplayName("should disable auto adjust DPI")
	void shouldDisableAutoAdjustDpi() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().autoAdjustDpi(false).build();

		assertThat(config.isAutoAdjustDpi()).isFalse();
	}

	@Test
	@DisplayName("should set min DPI")
	void shouldSetMinDpi() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().minDpi(200).build();

		assertThat(config.getMinDpi()).isEqualTo(200);
	}

	@Test
	@DisplayName("should set max DPI")
	void shouldSetMaxDpi() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().maxDpi(1200).build();

		assertThat(config.getMaxDpi()).isEqualTo(1200);
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().extractImages(true).targetDpi(400)
				.maxImageDimension(3000).autoAdjustDpi(true).minDpi(100).maxDpi(800).build();

		assertThat(config.isExtractImages()).isTrue();
		assertThat(config.getTargetDpi()).isEqualTo(400);
		assertThat(config.getMaxImageDimension()).isEqualTo(3000);
		assertThat(config.isAutoAdjustDpi()).isTrue();
		assertThat(config.getMinDpi()).isEqualTo(100);
		assertThat(config.getMaxDpi()).isEqualTo(800);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().extractImages(true).targetDpi(300)
				.maxImageDimension(2000).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("extract_images", true).containsEntry("target_dpi", 300)
				.containsEntry("max_image_dimension", 2000);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().extractImages(true).targetDpi(600)
				.maxImageDimension(3000).autoAdjustDpi(false).build();

		assertThat(config.isExtractImages()).isTrue();
		assertThat(config.getTargetDpi()).isEqualTo(600);
		assertThat(config.getMaxImageDimension()).isEqualTo(3000);
		assertThat(config.isAutoAdjustDpi()).isFalse();
	}

	@Test
	@DisplayName("should handle DPI range configurations")
	void shouldHandleDpiRange() {
		ImageExtractionConfig config = ImageExtractionConfig.builder().minDpi(100).targetDpi(300).maxDpi(1000).build();

		assertThat(config.getMinDpi()).isLessThan(config.getTargetDpi());
		assertThat(config.getTargetDpi()).isLessThan(config.getMaxDpi());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		ImageExtractionConfig config1 = ImageExtractionConfig.builder().targetDpi(300).build();
		ImageExtractionConfig config2 = ImageExtractionConfig.builder().targetDpi(600).build();

		assertThat(config1.getTargetDpi()).isNotEqualTo(config2.getTargetDpi());
	}
}

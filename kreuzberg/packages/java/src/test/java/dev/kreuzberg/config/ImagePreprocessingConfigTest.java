package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive ImagePreprocessingConfig tests.
 *
 * <p>
 * Tests for image preprocessing configuration including DPI, rotation,
 * deskewing, denoising, contrast enhancement, and binarization.
 */
@DisplayName("ImagePreprocessingConfig Tests")
final class ImagePreprocessingConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().build();

		assertThat(config.getTargetDpi()).isEqualTo(300);
		assertThat(config.isAutoRotate()).isTrue();
		assertThat(config.isDeskew()).isTrue();
		assertThat(config.isDenoise()).isFalse();
		assertThat(config.isContrastEnhance()).isTrue();
		assertThat(config.getBinarizationMethod()).isEqualTo("otsu");
		assertThat(config.isInvertColors()).isFalse();
	}

	@Test
	@DisplayName("should set target DPI")
	void shouldSetTargetDpi() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().targetDpi(600).build();

		assertThat(config.getTargetDpi()).isEqualTo(600);
	}

	@Test
	@DisplayName("should disable auto rotate")
	void shouldDisableAutoRotate() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().autoRotate(false).build();

		assertThat(config.isAutoRotate()).isFalse();
	}

	@Test
	@DisplayName("should disable deskew")
	void shouldDisableDeskew() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().deskew(false).build();

		assertThat(config.isDeskew()).isFalse();
	}

	@Test
	@DisplayName("should enable denoise")
	void shouldEnableDenoise() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().denoise(true).build();

		assertThat(config.isDenoise()).isTrue();
	}

	@Test
	@DisplayName("should disable contrast enhancement")
	void shouldDisableContrastEnhance() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().contrastEnhance(false).build();

		assertThat(config.isContrastEnhance()).isFalse();
	}

	@Test
	@DisplayName("should set binarization method")
	void shouldSetBinarizationMethod() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().binarizationMethod("adaptive").build();

		assertThat(config.getBinarizationMethod()).isEqualTo("adaptive");
	}

	@Test
	@DisplayName("should enable invert colors")
	void shouldEnableInvertColors() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().invertColors(true).build();

		assertThat(config.isInvertColors()).isTrue();
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().targetDpi(400).autoRotate(true)
				.deskew(true).denoise(true).contrastEnhance(false).binarizationMethod("adaptive").invertColors(true)
				.build();

		assertThat(config.getTargetDpi()).isEqualTo(400);
		assertThat(config.isAutoRotate()).isTrue();
		assertThat(config.isDeskew()).isTrue();
		assertThat(config.isDenoise()).isTrue();
		assertThat(config.isContrastEnhance()).isFalse();
		assertThat(config.getBinarizationMethod()).isEqualTo("adaptive");
		assertThat(config.isInvertColors()).isTrue();
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().targetDpi(300).autoRotate(true)
				.deskew(true).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("target_dpi", 300).containsEntry("auto_rotate", true).containsEntry("deskew",
				true);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().targetDpi(600).autoRotate(false)
				.deskew(false).denoise(true).build();

		assertThat(config.getTargetDpi()).isEqualTo(600);
		assertThat(config.isAutoRotate()).isFalse();
		assertThat(config.isDeskew()).isFalse();
		assertThat(config.isDenoise()).isTrue();
	}

	@Test
	@DisplayName("should handle multiple preprocessing options")
	void shouldHandleMultipleOptions() {
		ImagePreprocessingConfig config = ImagePreprocessingConfig.builder().targetDpi(400).denoise(true)
				.contrastEnhance(true).binarizationMethod("otsu").invertColors(false).build();

		assertThat(config.getTargetDpi()).isEqualTo(400);
		assertThat(config.isDenoise()).isTrue();
		assertThat(config.isContrastEnhance()).isTrue();
		assertThat(config.getBinarizationMethod()).isEqualTo("otsu");
		assertThat(config.isInvertColors()).isFalse();
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		ImagePreprocessingConfig config1 = ImagePreprocessingConfig.builder().targetDpi(300).build();
		ImagePreprocessingConfig config2 = ImagePreprocessingConfig.builder().targetDpi(600).build();

		assertThat(config1.getTargetDpi()).isNotEqualTo(config2.getTargetDpi());
	}
}

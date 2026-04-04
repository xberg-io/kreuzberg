package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive TokenReductionConfig tests.
 *
 * <p>
 * Tests for token reduction configuration including reduction mode and
 * important word preservation settings.
 */
@DisplayName("TokenReductionConfig Tests")
final class TokenReductionConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		TokenReductionConfig config = TokenReductionConfig.builder().build();

		assertThat(config.getMode()).isEqualTo("off");
		assertThat(config.isPreserveImportantWords()).isTrue();
	}

	@Test
	@DisplayName("should set mode to aggressive")
	void shouldSetModeAggressive() {
		TokenReductionConfig config = TokenReductionConfig.builder().mode("aggressive").build();

		assertThat(config.getMode()).isEqualTo("aggressive");
	}

	@Test
	@DisplayName("should set mode to moderate")
	void shouldSetModeModerate() {
		TokenReductionConfig config = TokenReductionConfig.builder().mode("moderate").build();

		assertThat(config.getMode()).isEqualTo("moderate");
	}

	@Test
	@DisplayName("should set mode to light")
	void shouldSetModeLight() {
		TokenReductionConfig config = TokenReductionConfig.builder().mode("light").build();

		assertThat(config.getMode()).isEqualTo("light");
	}

	@Test
	@DisplayName("should disable preserve important words")
	void shouldDisablePreserveImportantWords() {
		TokenReductionConfig config = TokenReductionConfig.builder().preserveImportantWords(false).build();

		assertThat(config.isPreserveImportantWords()).isFalse();
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		TokenReductionConfig config = TokenReductionConfig.builder().mode("aggressive").preserveImportantWords(true)
				.build();

		assertThat(config.getMode()).isEqualTo("aggressive");
		assertThat(config.isPreserveImportantWords()).isTrue();
	}

	@Test
	@DisplayName("should reject invalid mode")
	void shouldRejectInvalidMode() {
		assertThatThrownBy(() -> TokenReductionConfig.builder().mode("invalid_mode").build())
				.isInstanceOf(IllegalArgumentException.class);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		TokenReductionConfig config = TokenReductionConfig.builder().mode("moderate").preserveImportantWords(true)
				.build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("mode", "moderate").containsEntry("preserve_important_words", true);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		TokenReductionConfig config = TokenReductionConfig.builder().mode("aggressive").preserveImportantWords(false)
				.build();

		assertThat(config.getMode()).isEqualTo("aggressive");
		assertThat(config.isPreserveImportantWords()).isFalse();
	}

	@Test
	@DisplayName("should support different reduction levels")
	void shouldSupportDifferentReductionLevels() {
		TokenReductionConfig config1 = TokenReductionConfig.builder().mode("light").build();
		TokenReductionConfig config2 = TokenReductionConfig.builder().mode("moderate").build();
		TokenReductionConfig config3 = TokenReductionConfig.builder().mode("aggressive").build();

		assertThat(config1.getMode()).isNotEqualTo(config2.getMode());
		assertThat(config2.getMode()).isNotEqualTo(config3.getMode());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		TokenReductionConfig config1 = TokenReductionConfig.builder().mode("aggressive").build();
		TokenReductionConfig config2 = TokenReductionConfig.builder().mode("moderate").build();

		assertThat(config1.getMode()).isNotEqualTo(config2.getMode());
	}

	@Test
	@DisplayName("should handle preserve important words toggle")
	void shouldHandlePreserveToggle() {
		TokenReductionConfig configEnabled = TokenReductionConfig.builder().preserveImportantWords(true).build();
		TokenReductionConfig configDisabled = TokenReductionConfig.builder().preserveImportantWords(false).build();

		assertThat(configEnabled.isPreserveImportantWords()).isTrue();
		assertThat(configDisabled.isPreserveImportantWords()).isFalse();
	}
}

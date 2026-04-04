package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.Arrays;
import java.util.List;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive PostProcessorConfig tests.
 *
 * <p>
 * Tests for post-processor configuration including enable flag,
 * enabled/disabled processor lists, and processor management.
 */
@DisplayName("PostProcessorConfig Tests")
final class PostProcessorConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		PostProcessorConfig config = PostProcessorConfig.builder().build();

		assertThat(config.isEnabled()).isTrue();
		assertNull(config.getEnabledProcessors());
		assertNull(config.getDisabledProcessors());
	}

	@Test
	@DisplayName("should disable post-processor")
	void shouldDisablePostProcessor() {
		PostProcessorConfig config = PostProcessorConfig.builder().enabled(false).build();

		assertThat(config.isEnabled()).isFalse();
	}

	@Test
	@DisplayName("should set enabled processors list")
	void shouldSetEnabledProcessors() {
		List<String> processors = Arrays.asList("processor1", "processor2", "processor3");
		PostProcessorConfig config = PostProcessorConfig.builder().enabledProcessors(processors).build();

		assertThat(config.getEnabledProcessors()).containsExactlyElementsOf(processors);
	}

	@Test
	@DisplayName("should add single enabled processor")
	void shouldAddEnabledProcessor() {
		PostProcessorConfig config = PostProcessorConfig.builder().enabledProcessor("processor1")
				.enabledProcessor("processor2").build();

		assertThat(config.getEnabledProcessors()).contains("processor1", "processor2");
	}

	@Test
	@DisplayName("should set disabled processors list")
	void shouldSetDisabledProcessors() {
		List<String> processors = Arrays.asList("processor4", "processor5");
		PostProcessorConfig config = PostProcessorConfig.builder().disabledProcessors(processors).build();

		assertThat(config.getDisabledProcessors()).containsExactlyElementsOf(processors);
	}

	@Test
	@DisplayName("should add single disabled processor")
	void shouldAddDisabledProcessor() {
		PostProcessorConfig config = PostProcessorConfig.builder().disabledProcessor("processor1")
				.disabledProcessor("processor2").build();

		assertThat(config.getDisabledProcessors()).contains("processor1", "processor2");
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		List<String> enabled = Arrays.asList("p1", "p2");
		List<String> disabled = Arrays.asList("p3", "p4");

		PostProcessorConfig config = PostProcessorConfig.builder().enabled(true).enabledProcessors(enabled)
				.disabledProcessors(disabled).build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getEnabledProcessors()).containsExactlyElementsOf(enabled);
		assertThat(config.getDisabledProcessors()).containsExactlyElementsOf(disabled);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		List<String> enabled = Arrays.asList("p1", "p2");
		PostProcessorConfig config = PostProcessorConfig.builder().enabled(true).enabledProcessors(enabled).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("enabled", true).containsKey("enabled_processors");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		PostProcessorConfig config = PostProcessorConfig.builder().enabled(true).enabledProcessor("processor1")
				.enabledProcessor("processor2").disabledProcessor("processor3").build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getEnabledProcessors()).contains("processor1", "processor2");
		assertThat(config.getDisabledProcessors()).contains("processor3");
	}

	@Test
	@DisplayName("should handle enabled processors immutability")
	void shouldHandleEnabledProcessorsImmutability() {
		List<String> processors = Arrays.asList("p1", "p2");
		PostProcessorConfig config = PostProcessorConfig.builder().enabledProcessors(processors).build();

		assertThat(config.getEnabledProcessors()).isUnmodifiable();
	}

	@Test
	@DisplayName("should handle disabled processors immutability")
	void shouldHandleDisabledProcessorsImmutability() {
		List<String> processors = Arrays.asList("p1", "p2");
		PostProcessorConfig config = PostProcessorConfig.builder().disabledProcessors(processors).build();

		assertThat(config.getDisabledProcessors()).isUnmodifiable();
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		PostProcessorConfig config1 = PostProcessorConfig.builder().enabled(true).build();
		PostProcessorConfig config2 = PostProcessorConfig.builder().enabled(false).build();

		assertThat(config1.isEnabled()).isNotEqualTo(config2.isEnabled());
	}

	@Test
	@DisplayName("should support mixed enabled and disabled processors")
	void shouldSupportMixedProcessors() {
		PostProcessorConfig config = PostProcessorConfig.builder().enabled(true).enabledProcessor("normalize")
				.enabledProcessor("clean").disabledProcessor("compress").build();

		assertThat(config.getEnabledProcessors()).contains("normalize", "clean");
		assertThat(config.getDisabledProcessors()).contains("compress");
	}
}

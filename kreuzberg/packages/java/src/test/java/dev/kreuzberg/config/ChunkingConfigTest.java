package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.HashMap;
import java.util.Map;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive ChunkingConfig tests.
 *
 * <p>
 * Tests for text chunking configuration including max characters, overlap,
 * presets, and embedding configuration.
 */
@DisplayName("ChunkingConfig Tests")
final class ChunkingConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		ChunkingConfig config = ChunkingConfig.builder().build();

		assertThat(config.getMaxChars()).isEqualTo(1000);
		assertThat(config.getMaxOverlap()).isEqualTo(200);
		assertNull(config.getPreset());
		assertNull(config.getEmbedding());
		assertNull(config.getEnabled()); // Builder doesn't set default for enabled
	}

	@Test
	@DisplayName("should set max chars")
	void shouldSetMaxChars() {
		ChunkingConfig config = ChunkingConfig.builder().maxChars(2000).build();

		assertThat(config.getMaxChars()).isEqualTo(2000);
	}

	@Test
	@DisplayName("should set max overlap")
	void shouldSetMaxOverlap() {
		ChunkingConfig config = ChunkingConfig.builder().maxOverlap(500).build();

		assertThat(config.getMaxOverlap()).isEqualTo(500);
	}

	@Test
	@DisplayName("should set preset")
	void shouldSetPreset() {
		ChunkingConfig config = ChunkingConfig.builder().preset("sentence").build();

		assertThat(config.getPreset()).isEqualTo("sentence");
	}

	@Test
	@DisplayName("should set embedding configuration")
	void shouldSetEmbeddingConfig() {
		Map<String, Object> embedding = new HashMap<>();
		embedding.put("model", "all-MiniLM-L6-v2");

		ChunkingConfig config = ChunkingConfig.builder().embedding(embedding).build();

		assertNotNull(config.getEmbedding());
		assertThat(config.getEmbedding()).containsEntry("model", "all-MiniLM-L6-v2");
	}

	@Test
	@DisplayName("should set enabled flag")
	void shouldSetEnabledFlag() {
		ChunkingConfig config = ChunkingConfig.builder().enabled(false).build();

		assertThat(config.getEnabled()).isFalse();
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		Map<String, Object> embedding = new HashMap<>();
		embedding.put("model", "test-model");

		ChunkingConfig config = ChunkingConfig.builder().maxChars(1500).maxOverlap(300).preset("paragraph")
				.embedding(embedding).enabled(true).build();

		assertThat(config.getMaxChars()).isEqualTo(1500);
		assertThat(config.getMaxOverlap()).isEqualTo(300);
		assertThat(config.getPreset()).isEqualTo("paragraph");
		assertThat(config.getEmbedding()).containsEntry("model", "test-model");
		assertThat(config.getEnabled()).isTrue();
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		ChunkingConfig config = ChunkingConfig.builder().maxChars(1500).maxOverlap(300).preset("sentence").build();

		Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("max_chars", 1500).containsEntry("max_overlap", 300)
				.containsEntry("preset", "sentence").doesNotContainKey("enabled");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		ChunkingConfig config = ChunkingConfig.builder().maxChars(2000).maxOverlap(400).preset("document").build();

		assertThat(config.getMaxChars()).isEqualTo(2000);
		assertThat(config.getMaxOverlap()).isEqualTo(400);
		assertThat(config.getPreset()).isEqualTo("document");
	}

	@Test
	@DisplayName("should handle null preset")
	void shouldHandleNullPreset() {
		ChunkingConfig config = ChunkingConfig.builder().preset(null).build();

		assertNull(config.getPreset());
	}

	@Test
	@DisplayName("should handle null embedding")
	void shouldHandleNullEmbedding() {
		ChunkingConfig config = ChunkingConfig.builder().embedding(null).build();

		assertNull(config.getEmbedding());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		ChunkingConfig config1 = ChunkingConfig.builder().maxChars(1000).build();
		ChunkingConfig config2 = ChunkingConfig.builder().maxChars(2000).build();

		assertThat(config1.getMaxChars()).isNotEqualTo(config2.getMaxChars());
	}
}

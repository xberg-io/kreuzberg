package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.HashMap;
import java.util.Map;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive EmbeddingConfig tests.
 *
 * <p>
 * Tests for embedding generation configuration including model selection,
 * normalization, batch size, dimensions, caching, and download progress.
 */
@DisplayName("EmbeddingConfig Tests")
final class EmbeddingConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		EmbeddingConfig config = EmbeddingConfig.builder().build();

		Map<String, Object> model = config.getModel();
		assertThat(model).containsEntry("type", "preset").containsEntry("name", "balanced");
		assertNull(config.getNormalize()); // Builder doesn't set default
		assertThat(config.getBatchSize()).isEqualTo(32); // Builder has batchSize = 32 default
		assertNull(config.getDimensions());
		assertNull(config.getUseCache()); // Builder doesn't set default
		assertNull(config.getShowDownloadProgress()); // Builder doesn't set default
		assertNull(config.getCacheDir());
	}

	@Test
	@DisplayName("should set model preset")
	void shouldSetModelPreset() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("fast").build();

		Map<String, Object> model = config.getModel();
		assertThat(model).containsEntry("type", "preset").containsEntry("name", "fast");
	}

	@Test
	@DisplayName("should set normalize flag")
	void shouldSetNormalize() {
		EmbeddingConfig config = EmbeddingConfig.builder().normalize(false).build();

		assertThat(config.getNormalize()).isFalse();
	}

	@Test
	@DisplayName("should set batch size")
	void shouldSetBatchSize() {
		EmbeddingConfig config = EmbeddingConfig.builder().batchSize(64).build();

		assertThat(config.getBatchSize()).isEqualTo(64);
	}

	@Test
	@DisplayName("should set dimensions")
	void shouldSetDimensions() {
		EmbeddingConfig config = EmbeddingConfig.builder().dimensions(384).build();

		assertThat(config.getDimensions()).isEqualTo(384);
	}

	@Test
	@DisplayName("should set use cache flag")
	void shouldSetUseCache() {
		EmbeddingConfig config = EmbeddingConfig.builder().useCache(false).build();

		assertThat(config.getUseCache()).isFalse();
	}

	@Test
	@DisplayName("should set show download progress flag")
	void shouldSetShowDownloadProgress() {
		EmbeddingConfig config = EmbeddingConfig.builder().showDownloadProgress(true).build();

		assertThat(config.getShowDownloadProgress()).isTrue();
	}

	@Test
	@DisplayName("should set cache directory")
	void shouldSetCacheDir() {
		EmbeddingConfig config = EmbeddingConfig.builder().cacheDir("/custom/cache/path").build();

		assertThat(config.getCacheDir()).isEqualTo("/custom/cache/path");
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("quality").normalize(true).batchSize(128)
				.dimensions(768).useCache(true).showDownloadProgress(true).cacheDir("/tmp/embeddings").build();

		Map<String, Object> model = config.getModel();
		assertThat(model).containsEntry("type", "preset").containsEntry("name", "quality");
		assertThat(config.getNormalize()).isTrue();
		assertThat(config.getBatchSize()).isEqualTo(128);
		assertThat(config.getDimensions()).isEqualTo(768);
		assertThat(config.getUseCache()).isTrue();
		assertThat(config.getShowDownloadProgress()).isTrue();
		assertThat(config.getCacheDir()).isEqualTo("/tmp/embeddings");
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("balanced").normalize(true).batchSize(64)
				.dimensions(512).build();

		Map<String, Object> map = config.toMap();

		@SuppressWarnings("unchecked")
		Map<String, Object> model = (Map<String, Object>) map.get("model");
		assertThat(model).containsEntry("type", "preset").containsEntry("name", "balanced");
		assertThat(map).containsEntry("normalize", true).containsEntry("batch_size", 64)
				.containsEntry("dimensions", 512).doesNotContainKey("use_cache")
				.doesNotContainKey("show_download_progress");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("fast").batchSize(256).normalize(false).build();

		Map<String, Object> model = config.getModel();
		assertThat(model).containsEntry("type", "preset").containsEntry("name", "fast");
		assertThat(config.getBatchSize()).isEqualTo(256);
		assertThat(config.getNormalize()).isFalse();
	}

	@Test
	@DisplayName("should handle null model")
	void shouldHandleNullModel() {
		EmbeddingConfig config = EmbeddingConfig.builder().model(null).build();

		assertNull(config.getModel());
	}

	@Test
	@DisplayName("should handle null cache directory")
	void shouldHandleNullCacheDir() {
		EmbeddingConfig config = EmbeddingConfig.builder().cacheDir(null).build();

		assertNull(config.getCacheDir());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		EmbeddingConfig config1 = EmbeddingConfig.builder().preset("fast").build();
		EmbeddingConfig config2 = EmbeddingConfig.builder().preset("quality").build();

		assertThat(config1.getModel().get("name")).isNotEqualTo(config2.getModel().get("name"));
	}

	@Test
	@DisplayName("should support all preset names")
	void shouldSupportAllPresetNames() {
		EmbeddingConfig config1 = EmbeddingConfig.builder().preset("fast").dimensions(384).build();
		EmbeddingConfig config2 = EmbeddingConfig.builder().preset("balanced").dimensions(768).build();
		EmbeddingConfig config3 = EmbeddingConfig.builder().preset("quality").dimensions(1024).build();
		EmbeddingConfig config4 = EmbeddingConfig.builder().preset("multilingual").dimensions(768).build();

		assertThat(config1.getModel().get("name")).isEqualTo("fast");
		assertThat(config2.getModel().get("name")).isEqualTo("balanced");
		assertThat(config3.getModel().get("name")).isEqualTo("quality");
		assertThat(config4.getModel().get("name")).isEqualTo("multilingual");
	}

	@Test
	@DisplayName("should support various batch sizes")
	void shouldSupportVariousBatchSizes() {
		EmbeddingConfig config1 = EmbeddingConfig.builder().batchSize(1).build();
		EmbeddingConfig config2 = EmbeddingConfig.builder().batchSize(16).build();
		EmbeddingConfig config3 = EmbeddingConfig.builder().batchSize(64).build();
		EmbeddingConfig config4 = EmbeddingConfig.builder().batchSize(128).build();
		EmbeddingConfig config5 = EmbeddingConfig.builder().batchSize(512).build();

		assertThat(config1.getBatchSize()).isEqualTo(1);
		assertThat(config2.getBatchSize()).isEqualTo(16);
		assertThat(config3.getBatchSize()).isEqualTo(64);
		assertThat(config4.getBatchSize()).isEqualTo(128);
		assertThat(config5.getBatchSize()).isEqualTo(512);
	}

	@Test
	@DisplayName("should support common embedding dimensions")
	void shouldSupportCommonDimensions() {
		EmbeddingConfig config1 = EmbeddingConfig.builder().dimensions(384).build();
		EmbeddingConfig config2 = EmbeddingConfig.builder().dimensions(512).build();
		EmbeddingConfig config3 = EmbeddingConfig.builder().dimensions(768).build();
		EmbeddingConfig config4 = EmbeddingConfig.builder().dimensions(1024).build();
		EmbeddingConfig config5 = EmbeddingConfig.builder().dimensions(1536).build();

		assertThat(config1.getDimensions()).isEqualTo(384);
		assertThat(config2.getDimensions()).isEqualTo(512);
		assertThat(config3.getDimensions()).isEqualTo(768);
		assertThat(config4.getDimensions()).isEqualTo(1024);
		assertThat(config5.getDimensions()).isEqualTo(1536);
	}

	@Test
	@DisplayName("should create from map with model as Map")
	void shouldCreateFromMapWithModelAsMap() {
		Map<String, Object> modelMap = new HashMap<>();
		modelMap.put("type", "preset");
		modelMap.put("name", "fast");

		Map<String, Object> map = new HashMap<>();
		map.put("model", modelMap);
		map.put("normalize", null); // fromMap converts null to true
		map.put("batch_size", 64);
		map.put("dimensions", 512);
		map.put("use_cache", null); // fromMap converts null to false
		map.put("show_download_progress", null); // fromMap converts null to true
		map.put("cache_dir", "/test/cache");

		EmbeddingConfig config = EmbeddingConfig.fromMap(map);

		assertThat(config.getModel()).containsEntry("type", "preset").containsEntry("name", "fast");
		assertThat(config.getNormalize()).isTrue(); // fromMap(null) -> true
		assertThat(config.getBatchSize()).isEqualTo(64);
		assertThat(config.getDimensions()).isEqualTo(512);
		assertThat(config.getUseCache()).isFalse(); // fromMap(null) -> false
		assertThat(config.getShowDownloadProgress()).isTrue(); // fromMap(null) -> true
		assertThat(config.getCacheDir()).isEqualTo("/test/cache");
	}

	@Test
	@DisplayName("should create from map with model as string (legacy)")
	void shouldCreateFromMapWithModelAsString() {
		Map<String, Object> map = new HashMap<>();
		map.put("model", "balanced");
		map.put("batch_size", 32);

		EmbeddingConfig config = EmbeddingConfig.fromMap(map);

		assertThat(config.getModel()).containsEntry("type", "preset").containsEntry("name", "balanced");
		assertThat(config.getBatchSize()).isEqualTo(32);
		assertNull(config.getNormalize()); // fromMap doesn't set default when key is missing
	}

	@Test
	@DisplayName("should return null from map when input is null")
	void shouldReturnNullFromNullMap() {
		EmbeddingConfig config = EmbeddingConfig.fromMap(null);

		assertNull(config);
	}

	@Test
	@DisplayName("should handle missing fields in fromMap")
	void shouldHandleMissingFieldsInFromMap() {
		Map<String, Object> map = new HashMap<>();
		map.put("model", "fast");

		EmbeddingConfig config = EmbeddingConfig.fromMap(map);

		assertThat(config.getModel()).containsEntry("type", "preset").containsEntry("name", "fast");
		assertNull(config.getNormalize()); // fromMap doesn't set default when key is missing
		assertThat(config.getBatchSize()).isEqualTo(32); // Builder has batchSize = 32 default
	}

	@Test
	@DisplayName("should only include non-null fields in toMap")
	void shouldOnlyIncludeNonNullFieldsInToMap() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("balanced").normalize(true).build();

		Map<String, Object> map = config.toMap();

		assertThat(map).containsKey("model").containsKey("normalize").containsKey("batch_size")
				.doesNotContainKey("use_cache") // not set, so not in map
				.doesNotContainKey("show_download_progress") // not set, so not in map
				.doesNotContainKey("dimensions").doesNotContainKey("cache_dir");
	}

	@Test
	@DisplayName("should support integration with ChunkingConfig")
	void shouldSupportIntegrationWithChunkingConfig() {
		Map<String, Object> embeddingMap = new HashMap<>();
		embeddingMap.put("model", "quality");
		embeddingMap.put("batch_size", 64);
		embeddingMap.put("normalize", null);

		ChunkingConfig chunkingConfig = ChunkingConfig.builder().embedding(embeddingMap).build();

		assertNotNull(chunkingConfig.getEmbedding());
		assertThat(chunkingConfig.getEmbedding()).containsEntry("model", "quality").containsEntry("batch_size", 64)
				.containsEntry("normalize", null);
	}

	@Test
	@DisplayName("should handle Number conversion in fromMap")
	void shouldHandleNumberConversion() {
		Map<String, Object> map = new HashMap<>();
		map.put("model", "fast");
		map.put("batch_size", 64L); // Long instead of Integer

		EmbeddingConfig config = EmbeddingConfig.fromMap(map);

		assertThat(config.getModel()).containsEntry("name", "fast");
		assertThat(config.getBatchSize()).isEqualTo(64);
	}

	@Test
	@DisplayName("should configure for fast preset")
	void shouldConfigureForFastPreset() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("fast").dimensions(384).batchSize(64).normalize(true)
				.useCache(true).build();

		assertThat(config.getModel()).containsEntry("name", "fast");
		assertThat(config.getDimensions()).isEqualTo(384);
		assertThat(config.getBatchSize()).isEqualTo(64);
	}

	@Test
	@DisplayName("should configure for quality preset")
	void shouldConfigureForQualityPreset() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("quality").dimensions(1024).batchSize(32)
				.normalize(true).showDownloadProgress(true).build();

		assertThat(config.getModel()).containsEntry("name", "quality");
		assertThat(config.getDimensions()).isEqualTo(1024);
		assertThat(config.getBatchSize()).isEqualTo(32);
		assertThat(config.getShowDownloadProgress()).isTrue();
	}

	@Test
	@DisplayName("should configure for multilingual preset")
	void shouldConfigureForMultilingualPreset() {
		EmbeddingConfig config = EmbeddingConfig.builder().preset("multilingual").dimensions(768).batchSize(128)
				.normalize(true).useCache(true).cacheDir("/multilingual/cache").build();

		assertThat(config.getModel()).containsEntry("name", "multilingual");
		assertThat(config.getDimensions()).isEqualTo(768);
		assertThat(config.getBatchSize()).isEqualTo(128);
		assertThat(config.getCacheDir()).isEqualTo("/multilingual/cache");
	}
}

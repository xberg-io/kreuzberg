package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Embedding generation configuration for text chunks.
 *
 * <p>
 * Configures embedding generation using ONNX models via fastembed-rs.
 * Embeddings are useful for semantic search, clustering, and similarity
 * operations.
 *
 * <p>
 * Requires the embeddings feature to be enabled in the Rust core.
 *
 * @since 4.0.0
 */
public final class EmbeddingConfig {
	private final Map<String, Object> model;
	private final Boolean normalize;
	private final Integer batchSize;
	private final Integer dimensions;
	private final Boolean useCache;
	private final Boolean showDownloadProgress;
	private final String cacheDir;

	private EmbeddingConfig(Builder builder) {
		this.model = builder.model;
		this.normalize = builder.normalize;
		this.batchSize = builder.batchSize;
		this.dimensions = builder.dimensions;
		this.useCache = builder.useCache;
		this.showDownloadProgress = builder.showDownloadProgress;
		this.cacheDir = builder.cacheDir;
	}

	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Get the embedding model configuration.
	 *
	 * @return the model configuration map (with 'type' and 'name' keys), or null if
	 *         not set
	 */
	public Map<String, Object> getModel() {
		return model;
	}

	/**
	 * Check if embeddings should be normalized to unit length.
	 *
	 * @return true if normalization is enabled, false otherwise, or null if not set
	 */
	public Boolean getNormalize() {
		return normalize;
	}

	/**
	 * Get the batch size for processing.
	 *
	 * @return the batch size, or null if not set
	 */
	public Integer getBatchSize() {
		return batchSize;
	}

	/**
	 * Get the embedding dimensions.
	 *
	 * @return the dimensions, or null if not set
	 */
	public Integer getDimensions() {
		return dimensions;
	}

	/**
	 * Check if embeddings caching is enabled.
	 *
	 * @return true if caching is enabled, false otherwise, or null if not set
	 */
	public Boolean getUseCache() {
		return useCache;
	}

	/**
	 * Check if download progress should be shown.
	 *
	 * @return true if progress should be shown, false otherwise, or null if not set
	 */
	public Boolean getShowDownloadProgress() {
		return showDownloadProgress;
	}

	/**
	 * Get the custom cache directory for downloaded models.
	 *
	 * @return the cache directory path, or null if not set
	 */
	public String getCacheDir() {
		return cacheDir;
	}

	/**
	 * Convert configuration to map representation for serialization.
	 *
	 * @return map representation of this configuration
	 */
	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (model != null) {
			map.put("model", model);
		}
		if (normalize != null) {
			map.put("normalize", normalize);
		}
		if (batchSize != null) {
			map.put("batch_size", batchSize);
		}
		if (dimensions != null) {
			map.put("dimensions", dimensions);
		}
		if (useCache != null) {
			map.put("use_cache", useCache);
		}
		if (showDownloadProgress != null) {
			map.put("show_download_progress", showDownloadProgress);
		}
		if (cacheDir != null) {
			map.put("cache_dir", cacheDir);
		}
		return map;
	}

	/**
	 * Builder for constructing EmbeddingConfig instances.
	 */
	public static final class Builder {
		private Map<String, Object> model;
		private Boolean normalize;
		private Integer batchSize = 32;
		private Integer dimensions;
		private Boolean useCache;
		private Boolean showDownloadProgress;
		private String cacheDir;

		private Builder() {
			// Default to balanced preset
			this.model = new HashMap<>();
			this.model.put("type", "preset");
			this.model.put("name", "balanced");
		}

		/**
		 * Set the embedding model using a preset name.
		 *
		 * <p>
		 * Available presets:
		 * <ul>
		 * <li>'fast': AllMiniLML6V2Q (384 dims) - Quick prototyping, low-latency</li>
		 * <li>'balanced': BGEBaseENV15 (768 dims) - General-purpose RAG</li>
		 * <li>'quality': BGELargeENV15 (1024 dims) - High-quality embeddings</li>
		 * <li>'multilingual': MultilingualE5Base (768 dims) - Multi-language
		 * support</li>
		 * </ul>
		 *
		 * @param presetName
		 *            the preset name
		 * @return this builder
		 */
		public Builder preset(String presetName) {
			this.model = new HashMap<>();
			this.model.put("type", "preset");
			this.model.put("name", presetName);
			return this;
		}

		/**
		 * Set the embedding model configuration directly.
		 *
		 * <p>
		 * For advanced use cases. Use preset() for common configurations.
		 *
		 * @param model
		 *            the model configuration map
		 * @return this builder
		 */
		public Builder model(Map<String, Object> model) {
			this.model = model;
			return this;
		}

		/**
		 * Set whether to normalize embedding vectors to unit length.
		 *
		 * <p>
		 * Recommended for cosine similarity calculations.
		 *
		 * @param normalize
		 *            true to enable normalization, false otherwise
		 * @return this builder
		 */
		public Builder normalize(Boolean normalize) {
			this.normalize = normalize;
			return this;
		}

		/**
		 * Set the batch size for processing.
		 *
		 * <p>
		 * Higher values use more memory but may be faster. Valid range: 1-512
		 * (practical range)
		 * <p>
		 * Recommended values:
		 * <ul>
		 * <li>1-32: For memory-constrained environments</li>
		 * <li>32-128: Standard batch sizes for most systems</li>
		 * <li>128-512: For high-memory systems with GPU acceleration</li>
		 * </ul>
		 *
		 * @param batchSize
		 *            the batch size
		 * @return this builder
		 */
		public Builder batchSize(Integer batchSize) {
			this.batchSize = batchSize;
			return this;
		}

		/**
		 * Set the embedding dimensions.
		 *
		 * <p>
		 * The number of dimensions in the embedding vector. This should match the
		 * model's output dimensions.
		 *
		 * @param dimensions
		 *            the number of dimensions
		 * @return this builder
		 */
		public Builder dimensions(Integer dimensions) {
			this.dimensions = dimensions;
			return this;
		}

		/**
		 * Set whether to cache embeddings.
		 *
		 * <p>
		 * When enabled, embeddings are cached to avoid recomputation.
		 *
		 * @param useCache
		 *            true to enable caching, false otherwise
		 * @return this builder
		 */
		public Builder useCache(Boolean useCache) {
			this.useCache = useCache;
			return this;
		}

		/**
		 * Set whether to show download progress for embedding models.
		 *
		 * <p>
		 * Useful for large models on slow connections.
		 *
		 * @param showDownloadProgress
		 *            true to show progress, false otherwise
		 * @return this builder
		 */
		public Builder showDownloadProgress(Boolean showDownloadProgress) {
			this.showDownloadProgress = showDownloadProgress;
			return this;
		}

		/**
		 * Set a custom directory for caching downloaded models.
		 *
		 * <p>
		 * Defaults to ~/.cache/kreuzberg/embeddings/ if not specified.
		 *
		 * @param cacheDir
		 *            the cache directory path
		 * @return this builder
		 */
		public Builder cacheDir(String cacheDir) {
			this.cacheDir = cacheDir;
			return this;
		}

		/**
		 * Build the EmbeddingConfig instance.
		 *
		 * @return a new EmbeddingConfig instance
		 */
		public EmbeddingConfig build() {
			return new EmbeddingConfig(this);
		}
	}

	/**
	 * Create EmbeddingConfig from map representation.
	 *
	 * @param map
	 *            the map containing configuration values
	 * @return the parsed EmbeddingConfig, or null if map is null
	 */
	@SuppressWarnings("unchecked")
	static EmbeddingConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object modelValue = map.get("model");
		if (modelValue instanceof Map) {
			builder.model((Map<String, Object>) modelValue);
		} else if (modelValue instanceof String) {
			// Legacy: treat string as preset name
			builder.preset((String) modelValue);
		}
		if (map.containsKey("normalize")) {
			Object normalizeValue = map.get("normalize");
			if (normalizeValue instanceof Boolean) {
				builder.normalize((Boolean) normalizeValue);
			} else if (normalizeValue == null) {
				builder.normalize(true); // default when null
			}
		}
		Object batchSizeValue = map.get("batch_size");
		if (batchSizeValue instanceof Number) {
			builder.batchSize(((Number) batchSizeValue).intValue());
		}
		Object dimensionsValue = map.get("dimensions");
		if (dimensionsValue instanceof Number) {
			builder.dimensions(((Number) dimensionsValue).intValue());
		}
		if (map.containsKey("use_cache")) {
			Object useCacheValue = map.get("use_cache");
			if (useCacheValue instanceof Boolean) {
				builder.useCache((Boolean) useCacheValue);
			} else if (useCacheValue == null) {
				builder.useCache(false); // default when null
			}
		}
		if (map.containsKey("show_download_progress")) {
			Object showDownloadProgressValue = map.get("show_download_progress");
			if (showDownloadProgressValue instanceof Boolean) {
				builder.showDownloadProgress((Boolean) showDownloadProgressValue);
			} else if (showDownloadProgressValue == null) {
				builder.showDownloadProgress(true); // default when null
			}
		}
		Object cacheDirValue = map.get("cache_dir");
		if (cacheDirValue instanceof String) {
			builder.cacheDir((String) cacheDirValue);
		}
		return builder.build();
	}
}

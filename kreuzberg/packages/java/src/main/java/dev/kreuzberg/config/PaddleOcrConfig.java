package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Configuration for PaddleOCR backend.
 *
 * <p>
 * Configures PaddleOCR-specific parameters including language, caching, text
 * detection, recognition, and preprocessing options.
 *
 * @since 4.4.0
 */
public final class PaddleOcrConfig {
	private final String language;
	private final String cacheDir;
	private final Boolean useAngleCls;
	private final Boolean enableTableDetection;
	private final Double detDbThresh;
	private final Double detDbBoxThresh;
	private final Double detDbUnclipRatio;
	private final Integer detLimitSideLen;
	private final Integer recBatchNum;

	private PaddleOcrConfig(Builder builder) {
		this.language = builder.language;
		this.cacheDir = builder.cacheDir;
		this.useAngleCls = builder.useAngleCls;
		this.enableTableDetection = builder.enableTableDetection;
		this.detDbThresh = builder.detDbThresh;
		this.detDbBoxThresh = builder.detDbBoxThresh;
		this.detDbUnclipRatio = builder.detDbUnclipRatio;
		this.detLimitSideLen = builder.detLimitSideLen;
		this.recBatchNum = builder.recBatchNum;
	}

	public static Builder builder() {
		return new Builder();
	}

	public String getLanguage() {
		return language;
	}

	public String getCacheDir() {
		return cacheDir;
	}

	public Boolean getUseAngleCls() {
		return useAngleCls;
	}

	public Boolean getEnableTableDetection() {
		return enableTableDetection;
	}

	public Double getDetDbThresh() {
		return detDbThresh;
	}

	public Double getDetDbBoxThresh() {
		return detDbBoxThresh;
	}

	public Double getDetDbUnclipRatio() {
		return detDbUnclipRatio;
	}

	public Integer getDetLimitSideLen() {
		return detLimitSideLen;
	}

	public Integer getRecBatchNum() {
		return recBatchNum;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (language != null) {
			map.put("language", language);
		}
		if (cacheDir != null) {
			map.put("cache_dir", cacheDir);
		}
		if (useAngleCls != null) {
			map.put("use_angle_cls", useAngleCls);
		}
		if (enableTableDetection != null) {
			map.put("enable_table_detection", enableTableDetection);
		}
		if (detDbThresh != null) {
			map.put("det_db_thresh", detDbThresh);
		}
		if (detDbBoxThresh != null) {
			map.put("det_db_box_thresh", detDbBoxThresh);
		}
		if (detDbUnclipRatio != null) {
			map.put("det_db_unclip_ratio", detDbUnclipRatio);
		}
		if (detLimitSideLen != null) {
			map.put("det_limit_side_len", detLimitSideLen);
		}
		if (recBatchNum != null) {
			map.put("rec_batch_num", recBatchNum);
		}
		return map;
	}

	static PaddleOcrConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		if (map.get("language") instanceof String) {
			builder.language((String) map.get("language"));
		}
		if (map.get("cache_dir") instanceof String) {
			builder.cacheDir((String) map.get("cache_dir"));
		}
		if (map.get("use_angle_cls") instanceof Boolean) {
			builder.useAngleCls((Boolean) map.get("use_angle_cls"));
		}
		if (map.get("enable_table_detection") instanceof Boolean) {
			builder.enableTableDetection((Boolean) map.get("enable_table_detection"));
		}
		if (map.get("det_db_thresh") instanceof Number) {
			builder.detDbThresh(((Number) map.get("det_db_thresh")).doubleValue());
		}
		if (map.get("det_db_box_thresh") instanceof Number) {
			builder.detDbBoxThresh(((Number) map.get("det_db_box_thresh")).doubleValue());
		}
		if (map.get("det_db_unclip_ratio") instanceof Number) {
			builder.detDbUnclipRatio(((Number) map.get("det_db_unclip_ratio")).doubleValue());
		}
		if (map.get("det_limit_side_len") instanceof Number) {
			builder.detLimitSideLen(((Number) map.get("det_limit_side_len")).intValue());
		}
		if (map.get("rec_batch_num") instanceof Number) {
			builder.recBatchNum(((Number) map.get("rec_batch_num")).intValue());
		}
		return builder.build();
	}

	public static final class Builder {
		private String language;
		private String cacheDir;
		private Boolean useAngleCls;
		private Boolean enableTableDetection;
		private Double detDbThresh;
		private Double detDbBoxThresh;
		private Double detDbUnclipRatio;
		private Integer detLimitSideLen;
		private Integer recBatchNum;

		private Builder() {
		}

		public Builder language(String language) {
			this.language = language;
			return this;
		}

		public Builder cacheDir(String cacheDir) {
			this.cacheDir = cacheDir;
			return this;
		}

		public Builder useAngleCls(Boolean useAngleCls) {
			this.useAngleCls = useAngleCls;
			return this;
		}

		public Builder enableTableDetection(Boolean enableTableDetection) {
			this.enableTableDetection = enableTableDetection;
			return this;
		}

		public Builder detDbThresh(Double detDbThresh) {
			this.detDbThresh = detDbThresh;
			return this;
		}

		public Builder detDbBoxThresh(Double detDbBoxThresh) {
			this.detDbBoxThresh = detDbBoxThresh;
			return this;
		}

		public Builder detDbUnclipRatio(Double detDbUnclipRatio) {
			this.detDbUnclipRatio = detDbUnclipRatio;
			return this;
		}

		public Builder detLimitSideLen(Integer detLimitSideLen) {
			this.detLimitSideLen = detLimitSideLen;
			return this;
		}

		public Builder recBatchNum(Integer recBatchNum) {
			this.recBatchNum = recBatchNum;
			return this;
		}

		public PaddleOcrConfig build() {
			return new PaddleOcrConfig(this);
		}
	}
}

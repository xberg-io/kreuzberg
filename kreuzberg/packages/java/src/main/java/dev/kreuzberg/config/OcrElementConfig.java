package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Configuration for OCR element extraction.
 *
 * <p>
 * Configures how OCR elements are extracted and processed, including inclusion
 * filters, confidence thresholds, and hierarchy building options.
 *
 * @since 4.4.0
 */
public final class OcrElementConfig {
	private final boolean includeElements;
	private final String minLevel;
	private final double minConfidence;
	private final boolean buildHierarchy;

	private OcrElementConfig(Builder builder) {
		this.includeElements = builder.includeElements;
		this.minLevel = builder.minLevel;
		this.minConfidence = builder.minConfidence;
		this.buildHierarchy = builder.buildHierarchy;
	}

	public static Builder builder() {
		return new Builder();
	}

	public boolean isIncludeElements() {
		return includeElements;
	}

	public String getMinLevel() {
		return minLevel;
	}

	public double getMinConfidence() {
		return minConfidence;
	}

	public boolean isBuildHierarchy() {
		return buildHierarchy;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		map.put("include_elements", includeElements);
		if (minLevel != null) {
			map.put("min_level", minLevel);
		}
		map.put("min_confidence", minConfidence);
		map.put("build_hierarchy", buildHierarchy);
		return map;
	}

	static OcrElementConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		if (map.get("include_elements") instanceof Boolean) {
			builder.includeElements((Boolean) map.get("include_elements"));
		}
		if (map.get("min_level") instanceof String) {
			builder.minLevel((String) map.get("min_level"));
		}
		if (map.get("min_confidence") instanceof Number) {
			builder.minConfidence(((Number) map.get("min_confidence")).doubleValue());
		}
		if (map.get("build_hierarchy") instanceof Boolean) {
			builder.buildHierarchy((Boolean) map.get("build_hierarchy"));
		}
		return builder.build();
	}

	public static final class Builder {
		private boolean includeElements = false;
		private String minLevel = "word";
		private double minConfidence = 0.0;
		private boolean buildHierarchy = false;

		private Builder() {
		}

		public Builder includeElements(boolean includeElements) {
			this.includeElements = includeElements;
			return this;
		}

		public Builder minLevel(String minLevel) {
			this.minLevel = minLevel;
			return this;
		}

		public Builder minConfidence(double minConfidence) {
			this.minConfidence = minConfidence;
			return this;
		}

		public Builder buildHierarchy(boolean buildHierarchy) {
			this.buildHierarchy = buildHierarchy;
			return this;
		}

		public OcrElementConfig build() {
			return new OcrElementConfig(this);
		}
	}
}

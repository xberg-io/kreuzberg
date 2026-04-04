package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Hierarchy detection configuration for document structure analysis.
 *
 * @since 4.0.0
 */
public final class HierarchyConfig {
	private final boolean enabled;
	private final int kClusters;
	private final boolean includeBbox;
	private final Double ocrCoverageThreshold;

	private HierarchyConfig(Builder builder) {
		this.enabled = builder.enabled;
		this.kClusters = builder.kClusters;
		this.includeBbox = builder.includeBbox;
		this.ocrCoverageThreshold = builder.ocrCoverageThreshold;
	}

	public static Builder builder() {
		return new Builder();
	}

	public boolean isEnabled() {
		return enabled;
	}

	public int getKClusters() {
		return kClusters;
	}

	public boolean isIncludeBbox() {
		return includeBbox;
	}

	public Double getOcrCoverageThreshold() {
		return ocrCoverageThreshold;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		map.put("enabled", enabled);
		map.put("k_clusters", kClusters);
		map.put("include_bbox", includeBbox);
		if (ocrCoverageThreshold != null) {
			map.put("ocr_coverage_threshold", ocrCoverageThreshold);
		}
		return map;
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) {
			return true;
		}
		if (o == null || getClass() != o.getClass()) {
			return false;
		}
		HierarchyConfig that = (HierarchyConfig) o;
		return enabled == that.enabled && kClusters == that.kClusters && includeBbox == that.includeBbox
				&& java.util.Objects.equals(ocrCoverageThreshold, that.ocrCoverageThreshold);
	}

	@Override
	public int hashCode() {
		return java.util.Objects.hash(enabled, kClusters, includeBbox, ocrCoverageThreshold);
	}

	public static final class Builder {
		private boolean enabled = true;
		private int kClusters = 6;
		private boolean includeBbox = true;
		private Double ocrCoverageThreshold;

		private Builder() {
		}

		public Builder enabled(boolean enabled) {
			this.enabled = enabled;
			return this;
		}

		public Builder kClusters(int kClusters) {
			this.kClusters = kClusters;
			return this;
		}

		public Builder includeBbox(boolean includeBbox) {
			this.includeBbox = includeBbox;
			return this;
		}

		public Builder ocrCoverageThreshold(Double ocrCoverageThreshold) {
			this.ocrCoverageThreshold = ocrCoverageThreshold;
			return this;
		}

		public HierarchyConfig build() {
			return new HierarchyConfig(this);
		}
	}

	static HierarchyConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object enabledValue = map.get("enabled");
		if (enabledValue instanceof Boolean) {
			builder.enabled((Boolean) enabledValue);
		}
		Object kClustersValue = map.get("k_clusters");
		if (kClustersValue instanceof Number) {
			builder.kClusters(((Number) kClustersValue).intValue());
		}
		Object includeBboxValue = map.get("include_bbox");
		if (includeBboxValue instanceof Boolean) {
			builder.includeBbox((Boolean) includeBboxValue);
		}
		Object ocrCoverageThresholdValue = map.get("ocr_coverage_threshold");
		if (ocrCoverageThresholdValue instanceof Number) {
			builder.ocrCoverageThreshold(((Number) ocrCoverageThresholdValue).doubleValue());
		}
		return builder.build();
	}
}

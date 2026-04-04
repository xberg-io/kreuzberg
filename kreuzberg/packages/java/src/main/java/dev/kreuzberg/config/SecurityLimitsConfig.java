package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Security limits for archive and document extraction.
 *
 * <p>
 * Controls thresholds to prevent resource exhaustion attacks such as
 * decompression bombs, deeply nested archives, and oversized content.
 *
 * @since 4.0.0
 */
public final class SecurityLimitsConfig {
	private final Long maxArchiveSize;
	private final Long maxCompressionRatio;
	private final Long maxFilesInArchive;
	private final Long maxNestingDepth;
	private final Long maxEntityLength;
	private final Long maxContentSize;
	private final Long maxIterations;
	private final Long maxXmlDepth;
	private final Long maxTableCells;

	private SecurityLimitsConfig(Builder builder) {
		this.maxArchiveSize = builder.maxArchiveSize;
		this.maxCompressionRatio = builder.maxCompressionRatio;
		this.maxFilesInArchive = builder.maxFilesInArchive;
		this.maxNestingDepth = builder.maxNestingDepth;
		this.maxEntityLength = builder.maxEntityLength;
		this.maxContentSize = builder.maxContentSize;
		this.maxIterations = builder.maxIterations;
		this.maxXmlDepth = builder.maxXmlDepth;
		this.maxTableCells = builder.maxTableCells;
	}

	public static Builder builder() {
		return new Builder();
	}

	/** Maximum allowed archive size in bytes. */
	public Long getMaxArchiveSize() {
		return maxArchiveSize;
	}

	/** Maximum allowed compression ratio (uncompressed / compressed). */
	public Long getMaxCompressionRatio() {
		return maxCompressionRatio;
	}

	/** Maximum number of files allowed inside an archive. */
	public Long getMaxFilesInArchive() {
		return maxFilesInArchive;
	}

	/** Maximum nesting depth for recursive archive extraction. */
	public Long getMaxNestingDepth() {
		return maxNestingDepth;
	}

	/** Maximum length of a single XML/HTML entity. */
	public Long getMaxEntityLength() {
		return maxEntityLength;
	}

	/** Maximum total content size in bytes after extraction. */
	public Long getMaxContentSize() {
		return maxContentSize;
	}

	/** Maximum number of processing iterations. */
	public Long getMaxIterations() {
		return maxIterations;
	}

	/** Maximum XML document nesting depth. */
	public Long getMaxXmlDepth() {
		return maxXmlDepth;
	}

	/** Maximum number of cells in a single table. */
	public Long getMaxTableCells() {
		return maxTableCells;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (maxArchiveSize != null) {
			map.put("max_archive_size", maxArchiveSize);
		}
		if (maxCompressionRatio != null) {
			map.put("max_compression_ratio", maxCompressionRatio);
		}
		if (maxFilesInArchive != null) {
			map.put("max_files_in_archive", maxFilesInArchive);
		}
		if (maxNestingDepth != null) {
			map.put("max_nesting_depth", maxNestingDepth);
		}
		if (maxEntityLength != null) {
			map.put("max_entity_length", maxEntityLength);
		}
		if (maxContentSize != null) {
			map.put("max_content_size", maxContentSize);
		}
		if (maxIterations != null) {
			map.put("max_iterations", maxIterations);
		}
		if (maxXmlDepth != null) {
			map.put("max_xml_depth", maxXmlDepth);
		}
		if (maxTableCells != null) {
			map.put("max_table_cells", maxTableCells);
		}
		return map;
	}

	public static SecurityLimitsConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		if (map.get("max_archive_size") instanceof Number) {
			builder.maxArchiveSize(((Number) map.get("max_archive_size")).longValue());
		}
		if (map.get("max_compression_ratio") instanceof Number) {
			builder.maxCompressionRatio(((Number) map.get("max_compression_ratio")).longValue());
		}
		if (map.get("max_files_in_archive") instanceof Number) {
			builder.maxFilesInArchive(((Number) map.get("max_files_in_archive")).longValue());
		}
		if (map.get("max_nesting_depth") instanceof Number) {
			builder.maxNestingDepth(((Number) map.get("max_nesting_depth")).longValue());
		}
		if (map.get("max_entity_length") instanceof Number) {
			builder.maxEntityLength(((Number) map.get("max_entity_length")).longValue());
		}
		if (map.get("max_content_size") instanceof Number) {
			builder.maxContentSize(((Number) map.get("max_content_size")).longValue());
		}
		if (map.get("max_iterations") instanceof Number) {
			builder.maxIterations(((Number) map.get("max_iterations")).longValue());
		}
		if (map.get("max_xml_depth") instanceof Number) {
			builder.maxXmlDepth(((Number) map.get("max_xml_depth")).longValue());
		}
		if (map.get("max_table_cells") instanceof Number) {
			builder.maxTableCells(((Number) map.get("max_table_cells")).longValue());
		}
		return builder.build();
	}

	public static final class Builder {
		private Long maxArchiveSize;
		private Long maxCompressionRatio;
		private Long maxFilesInArchive;
		private Long maxNestingDepth;
		private Long maxEntityLength;
		private Long maxContentSize;
		private Long maxIterations;
		private Long maxXmlDepth;
		private Long maxTableCells;

		private Builder() {
		}

		public Builder maxArchiveSize(Long maxArchiveSize) {
			this.maxArchiveSize = maxArchiveSize;
			return this;
		}

		public Builder maxCompressionRatio(Long maxCompressionRatio) {
			this.maxCompressionRatio = maxCompressionRatio;
			return this;
		}

		public Builder maxFilesInArchive(Long maxFilesInArchive) {
			this.maxFilesInArchive = maxFilesInArchive;
			return this;
		}

		public Builder maxNestingDepth(Long maxNestingDepth) {
			this.maxNestingDepth = maxNestingDepth;
			return this;
		}

		public Builder maxEntityLength(Long maxEntityLength) {
			this.maxEntityLength = maxEntityLength;
			return this;
		}

		public Builder maxContentSize(Long maxContentSize) {
			this.maxContentSize = maxContentSize;
			return this;
		}

		public Builder maxIterations(Long maxIterations) {
			this.maxIterations = maxIterations;
			return this;
		}

		public Builder maxXmlDepth(Long maxXmlDepth) {
			this.maxXmlDepth = maxXmlDepth;
			return this;
		}

		public Builder maxTableCells(Long maxTableCells) {
			this.maxTableCells = maxTableCells;
			return this;
		}

		public SecurityLimitsConfig build() {
			return new SecurityLimitsConfig(this);
		}
	}
}

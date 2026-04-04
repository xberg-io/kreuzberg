package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Configuration for tree-sitter code analysis processing options.
 *
 * @since 4.7.0
 */
public final class TreeSitterProcessConfig {
	private final Boolean structure;
	private final Boolean imports;
	private final Boolean exports;
	private final Boolean comments;
	private final Boolean docstrings;
	private final Boolean symbols;
	private final Boolean diagnostics;
	private final Integer chunkMaxSize;
	private final String contentMode;

	private TreeSitterProcessConfig(Builder builder) {
		this.structure = builder.structure;
		this.imports = builder.imports;
		this.exports = builder.exports;
		this.comments = builder.comments;
		this.docstrings = builder.docstrings;
		this.symbols = builder.symbols;
		this.diagnostics = builder.diagnostics;
		this.chunkMaxSize = builder.chunkMaxSize;
		this.contentMode = builder.contentMode;
	}

	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Whether to extract code structure information.
	 *
	 * @return true if structure extraction is enabled, or null if not set
	 */
	public Boolean getStructure() {
		return structure;
	}

	/**
	 * Whether to extract import statements.
	 *
	 * @return true if import extraction is enabled, or null if not set
	 */
	public Boolean getImports() {
		return imports;
	}

	/**
	 * Whether to extract export statements.
	 *
	 * @return true if export extraction is enabled, or null if not set
	 */
	public Boolean getExports() {
		return exports;
	}

	/**
	 * Whether to extract comments.
	 *
	 * @return true if comment extraction is enabled, or null if not set
	 */
	public Boolean getComments() {
		return comments;
	}

	/**
	 * Whether to extract docstrings.
	 *
	 * @return true if docstring extraction is enabled, or null if not set
	 */
	public Boolean getDocstrings() {
		return docstrings;
	}

	/**
	 * Whether to extract symbol definitions.
	 *
	 * @return true if symbol extraction is enabled, or null if not set
	 */
	public Boolean getSymbols() {
		return symbols;
	}

	/**
	 * Whether to extract diagnostics information.
	 *
	 * @return true if diagnostics extraction is enabled, or null if not set
	 */
	public Boolean getDiagnostics() {
		return diagnostics;
	}

	/**
	 * Get the maximum size of code chunks for processing.
	 *
	 * @return the maximum chunk size, or null if not set
	 */
	public Integer getChunkMaxSize() {
		return chunkMaxSize;
	}

	/**
	 * Get the content rendering mode for code extraction.
	 *
	 * @return the content mode ("chunks", "raw", or "structure"), or null if not
	 *         set
	 */
	public String getContentMode() {
		return contentMode;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (structure != null) {
			map.put("structure", structure);
		}
		if (imports != null) {
			map.put("imports", imports);
		}
		if (exports != null) {
			map.put("exports", exports);
		}
		if (comments != null) {
			map.put("comments", comments);
		}
		if (docstrings != null) {
			map.put("docstrings", docstrings);
		}
		if (symbols != null) {
			map.put("symbols", symbols);
		}
		if (diagnostics != null) {
			map.put("diagnostics", diagnostics);
		}
		if (chunkMaxSize != null) {
			map.put("chunk_max_size", chunkMaxSize);
		}
		if (contentMode != null) {
			map.put("content_mode", contentMode);
		}
		return map;
	}

	public static final class Builder {
		private Boolean structure;
		private Boolean imports;
		private Boolean exports;
		private Boolean comments;
		private Boolean docstrings;
		private Boolean symbols;
		private Boolean diagnostics;
		private Integer chunkMaxSize;
		private String contentMode;

		private Builder() {
		}

		/**
		 * Enable or disable code structure extraction.
		 *
		 * @param structure
		 *            true to enable structure extraction
		 * @return this builder for chaining
		 */
		public Builder structure(Boolean structure) {
			this.structure = structure;
			return this;
		}

		/**
		 * Enable or disable import statement extraction.
		 *
		 * @param imports
		 *            true to enable import extraction
		 * @return this builder for chaining
		 */
		public Builder imports(Boolean imports) {
			this.imports = imports;
			return this;
		}

		/**
		 * Enable or disable export statement extraction.
		 *
		 * @param exports
		 *            true to enable export extraction
		 * @return this builder for chaining
		 */
		public Builder exports(Boolean exports) {
			this.exports = exports;
			return this;
		}

		/**
		 * Enable or disable comment extraction.
		 *
		 * @param comments
		 *            true to enable comment extraction
		 * @return this builder for chaining
		 */
		public Builder comments(Boolean comments) {
			this.comments = comments;
			return this;
		}

		/**
		 * Enable or disable docstring extraction.
		 *
		 * @param docstrings
		 *            true to enable docstring extraction
		 * @return this builder for chaining
		 */
		public Builder docstrings(Boolean docstrings) {
			this.docstrings = docstrings;
			return this;
		}

		/**
		 * Enable or disable symbol definition extraction.
		 *
		 * @param symbols
		 *            true to enable symbol extraction
		 * @return this builder for chaining
		 */
		public Builder symbols(Boolean symbols) {
			this.symbols = symbols;
			return this;
		}

		/**
		 * Enable or disable diagnostics extraction.
		 *
		 * @param diagnostics
		 *            true to enable diagnostics extraction
		 * @return this builder for chaining
		 */
		public Builder diagnostics(Boolean diagnostics) {
			this.diagnostics = diagnostics;
			return this;
		}

		/**
		 * Set the maximum size of code chunks for processing.
		 *
		 * @param chunkMaxSize
		 *            the maximum chunk size
		 * @return this builder for chaining
		 */
		public Builder chunkMaxSize(Integer chunkMaxSize) {
			this.chunkMaxSize = chunkMaxSize;
			return this;
		}

		/**
		 * Set the content rendering mode for code extraction.
		 *
		 * @param contentMode
		 *            the content mode ("chunks", "raw", or "structure")
		 * @return this builder for chaining
		 */
		public Builder contentMode(String contentMode) {
			this.contentMode = contentMode;
			return this;
		}

		public TreeSitterProcessConfig build() {
			return new TreeSitterProcessConfig(this);
		}
	}

	static TreeSitterProcessConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		if (map.containsKey("structure")) {
			Object value = map.get("structure");
			if (value instanceof Boolean) {
				builder.structure((Boolean) value);
			}
		}
		if (map.containsKey("imports")) {
			Object value = map.get("imports");
			if (value instanceof Boolean) {
				builder.imports((Boolean) value);
			}
		}
		if (map.containsKey("exports")) {
			Object value = map.get("exports");
			if (value instanceof Boolean) {
				builder.exports((Boolean) value);
			}
		}
		if (map.containsKey("comments")) {
			Object value = map.get("comments");
			if (value instanceof Boolean) {
				builder.comments((Boolean) value);
			}
		}
		if (map.containsKey("docstrings")) {
			Object value = map.get("docstrings");
			if (value instanceof Boolean) {
				builder.docstrings((Boolean) value);
			}
		}
		if (map.containsKey("symbols")) {
			Object value = map.get("symbols");
			if (value instanceof Boolean) {
				builder.symbols((Boolean) value);
			}
		}
		if (map.containsKey("diagnostics")) {
			Object value = map.get("diagnostics");
			if (value instanceof Boolean) {
				builder.diagnostics((Boolean) value);
			}
		}
		Object chunkMaxSizeValue = map.get("chunk_max_size");
		if (chunkMaxSizeValue instanceof Number) {
			builder.chunkMaxSize(((Number) chunkMaxSizeValue).intValue());
		}
		Object contentModeValue = map.get("content_mode");
		if (contentModeValue instanceof String) {
			builder.contentMode((String) contentModeValue);
		}
		return builder.build();
	}
}

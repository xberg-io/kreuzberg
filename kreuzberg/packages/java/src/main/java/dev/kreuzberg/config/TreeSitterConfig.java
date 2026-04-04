package dev.kreuzberg.config;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Configuration for tree-sitter language pack integration.
 *
 * @since 4.7.0
 */
public final class TreeSitterConfig {
	private final Boolean enabled;
	private final String cacheDir;
	private final List<String> languages;
	private final List<String> groups;
	private final TreeSitterProcessConfig process;

	private TreeSitterConfig(Builder builder) {
		this.enabled = builder.enabled;
		this.cacheDir = builder.cacheDir;
		this.languages = builder.languages;
		this.groups = builder.groups;
		this.process = builder.process;
	}

	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Whether code intelligence processing is enabled.
	 *
	 * @return true if enabled, or null if not set (defaults to true)
	 */
	public Boolean getEnabled() {
		return enabled;
	}

	/**
	 * Get the directory for caching tree-sitter language packs.
	 *
	 * @return the cache directory, or null if not set
	 */
	public String getCacheDir() {
		return cacheDir;
	}

	/**
	 * Get the list of specific languages to enable for tree-sitter parsing.
	 *
	 * @return the list of languages, or null if not set
	 */
	public List<String> getLanguages() {
		return languages;
	}

	/**
	 * Get the list of language groups to enable for tree-sitter parsing.
	 *
	 * @return the list of language groups, or null if not set
	 */
	public List<String> getGroups() {
		return groups;
	}

	/**
	 * Get the processing options for tree-sitter code analysis.
	 *
	 * @return the process configuration, or null if not set
	 */
	public TreeSitterProcessConfig getProcess() {
		return process;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (enabled != null) {
			map.put("enabled", enabled);
		}
		if (cacheDir != null) {
			map.put("cache_dir", cacheDir);
		}
		if (languages != null) {
			map.put("languages", languages);
		}
		if (groups != null) {
			map.put("groups", groups);
		}
		if (process != null) {
			map.put("process", process.toMap());
		}
		return map;
	}

	public static final class Builder {
		private Boolean enabled;
		private String cacheDir;
		private List<String> languages;
		private List<String> groups;
		private TreeSitterProcessConfig process;

		private Builder() {
		}

		/**
		 * Enable or disable code intelligence processing.
		 *
		 * @param enabled
		 *            true to enable, false to disable
		 * @return this builder for chaining
		 */
		public Builder enabled(Boolean enabled) {
			this.enabled = enabled;
			return this;
		}

		/**
		 * Set the directory for caching tree-sitter language packs.
		 *
		 * @param cacheDir
		 *            the cache directory path
		 * @return this builder for chaining
		 */
		public Builder cacheDir(String cacheDir) {
			this.cacheDir = cacheDir;
			return this;
		}

		/**
		 * Set the list of specific languages to enable for tree-sitter parsing.
		 *
		 * @param languages
		 *            the list of language identifiers
		 * @return this builder for chaining
		 */
		public Builder languages(List<String> languages) {
			this.languages = languages;
			return this;
		}

		/**
		 * Set the list of language groups to enable for tree-sitter parsing.
		 *
		 * @param groups
		 *            the list of language group identifiers
		 * @return this builder for chaining
		 */
		public Builder groups(List<String> groups) {
			this.groups = groups;
			return this;
		}

		/**
		 * Set the processing options for tree-sitter code analysis.
		 *
		 * @param process
		 *            the process configuration
		 * @return this builder for chaining
		 */
		public Builder process(TreeSitterProcessConfig process) {
			this.process = process;
			return this;
		}

		public TreeSitterConfig build() {
			return new TreeSitterConfig(this);
		}
	}

	@SuppressWarnings("unchecked")
	static TreeSitterConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object enabledValue = map.get("enabled");
		if (enabledValue instanceof Boolean) {
			builder.enabled((Boolean) enabledValue);
		}
		Object cacheDirValue = map.get("cache_dir");
		if (cacheDirValue instanceof String) {
			builder.cacheDir((String) cacheDirValue);
		}
		Object languagesValue = map.get("languages");
		if (languagesValue instanceof List) {
			List<String> langs = new ArrayList<>();
			for (Object item : (List<?>) languagesValue) {
				if (item instanceof String) {
					langs.add((String) item);
				}
			}
			builder.languages(langs);
		}
		Object groupsValue = map.get("groups");
		if (groupsValue instanceof List) {
			List<String> grps = new ArrayList<>();
			for (Object item : (List<?>) groupsValue) {
				if (item instanceof String) {
					grps.add((String) item);
				}
			}
			builder.groups(grps);
		}
		Map<String, Object> processMap = map.get("process") instanceof Map
				? (Map<String, Object>) map.get("process")
				: null;
		if (processMap != null) {
			builder.process(TreeSitterProcessConfig.fromMap(processMap));
		}
		return builder.build();
	}
}

package dev.kreuzberg.config;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Font configuration for document processing.
 *
 * @since 4.0.0
 */
public final class FontConfig {
	private final boolean enabled;
	private final List<String> customFontDirs;

	private FontConfig(Builder builder) {
		this.enabled = builder.enabled;
		this.customFontDirs = builder.customFontDirs != null
				? Collections.unmodifiableList(new ArrayList<>(builder.customFontDirs))
				: null;
	}

	public static Builder builder() {
		return new Builder();
	}

	public boolean isEnabled() {
		return enabled;
	}

	public List<String> getCustomFontDirs() {
		return customFontDirs;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		map.put("enabled", enabled);
		if (customFontDirs != null && !customFontDirs.isEmpty()) {
			map.put("custom_font_dirs", customFontDirs);
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
		FontConfig that = (FontConfig) o;
		return enabled == that.enabled && java.util.Objects.equals(customFontDirs, that.customFontDirs);
	}

	@Override
	public int hashCode() {
		return java.util.Objects.hash(enabled, customFontDirs);
	}

	public static final class Builder {
		private boolean enabled = true;
		private List<String> customFontDirs;

		private Builder() {
		}

		public Builder enabled(boolean enabled) {
			this.enabled = enabled;
			return this;
		}

		public Builder addCustomFontDir(String path) {
			if (this.customFontDirs == null) {
				this.customFontDirs = new ArrayList<>();
			}
			this.customFontDirs.add(path);
			return this;
		}

		public Builder customFontDirs(List<String> dirs) {
			if (dirs == null) {
				this.customFontDirs = null;
			} else {
				this.customFontDirs = new ArrayList<>(dirs);
			}
			return this;
		}

		public FontConfig build() {
			return new FontConfig(this);
		}
	}

	static FontConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object enabledValue = map.get("enabled");
		if (enabledValue instanceof Boolean) {
			builder.enabled((Boolean) enabledValue);
		}
		Object customFontDirsValue = map.get("custom_font_dirs");
		if (customFontDirsValue instanceof Iterable) {
			@SuppressWarnings("unchecked")
			Iterable<Object> iterable = (Iterable<Object>) customFontDirsValue;
			List<String> dirs = new ArrayList<>();
			for (Object entry : iterable) {
				if (entry instanceof String) {
					dirs.add((String) entry);
				}
			}
			builder.customFontDirs(dirs);
		}
		return builder.build();
	}
}

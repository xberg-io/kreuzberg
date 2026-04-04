package dev.kreuzberg;

import dev.kreuzberg.config.FileExtractionConfig;
import java.util.Objects;

/**
 * A file path paired with an optional per-file extraction config override.
 *
 * @param path
 *            the file path to extract
 * @param config
 *            optional per-file config overrides (null = use batch default)
 * @since 4.6.0
 */
public record FileItem(String path, FileExtractionConfig config) {
	public FileItem {
		Objects.requireNonNull(path, "path must not be null");
		if (path.isBlank()) {
			throw new IllegalArgumentException("path must not be blank");
		}
	}

	/**
	 * Create a FileItem with no per-file config override.
	 *
	 * @param path
	 *            the file path
	 */
	public FileItem(String path) {
		this(path, null);
	}
}

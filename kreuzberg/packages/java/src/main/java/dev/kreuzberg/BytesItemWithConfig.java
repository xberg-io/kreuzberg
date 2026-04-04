package dev.kreuzberg;

import dev.kreuzberg.config.FileExtractionConfig;
import java.util.Objects;

/**
 * In-memory document data with MIME type and optional per-file config override.
 *
 * @param data
 *            the document bytes
 * @param mimeType
 *            the MIME type of the document
 * @param config
 *            optional per-file config overrides (null = use batch default)
 * @since 4.6.0
 */
public record BytesItemWithConfig(byte[] data, String mimeType, FileExtractionConfig config) {
	public BytesItemWithConfig {
		if (data == null || data.length == 0) {
			throw new IllegalArgumentException("data must not be null or empty");
		}
		Objects.requireNonNull(mimeType, "mimeType must not be null");
		if (mimeType.isBlank()) {
			throw new IllegalArgumentException("mimeType must not be blank");
		}
	}

	/**
	 * Create a BytesItemWithConfig with no per-file config override.
	 *
	 * @param data
	 *            the document bytes
	 * @param mimeType
	 *            the MIME type
	 */
	public BytesItemWithConfig(byte[] data, String mimeType) {
		this(data, mimeType, null);
	}
}

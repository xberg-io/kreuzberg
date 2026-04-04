package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Image type classification.
 *
 * <p>
 * Classifies images as data URIs, inline SVG, external URLs, or relative paths.
 *
 * @since 0.8.0
 */
public enum ImageType {
	/** Data URI image */
	DATA_URI("data_uri"),

	/** Inline SVG */
	INLINE_SVG("inline_svg"),

	/** External image URL */
	EXTERNAL("external"),

	/** Relative path image */
	RELATIVE("relative");

	private final String wireValue;

	ImageType(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this image type.
	 *
	 * @return wire value used in serialization (snake_case)
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse an ImageType from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (snake_case string)
	 * @return the corresponding ImageType
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	@JsonCreator
	public static ImageType fromWireValue(String wireValue) {
		for (ImageType type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown ImageType: " + wireValue);
	}
}

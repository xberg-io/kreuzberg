package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Text direction enumeration for HTML documents.
 *
 * <p>
 * Specifies the directionality of document text (left-to-right, right-to-left,
 * or automatic detection).
 *
 * @since 0.8.0
 */
public enum TextDirection {
	/** Left-to-right text direction */
	LEFT_TO_RIGHT("ltr"),

	/** Right-to-left text direction */
	RIGHT_TO_LEFT("rtl"),

	/** Automatic text direction detection */
	AUTO("auto");

	private final String wireValue;

	TextDirection(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this text direction.
	 *
	 * @return wire value used in serialization (lowercase)
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse a TextDirection from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (lowercase string)
	 * @return the corresponding TextDirection
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	@JsonCreator
	public static TextDirection fromWireValue(String wireValue) {
		for (TextDirection type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown TextDirection: " + wireValue);
	}
}

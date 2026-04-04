package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Link type classification for HTML links.
 *
 * <p>
 * Classifies links as anchors, internal/external references, email, phone, or
 * other types.
 *
 * @since 0.8.0
 */
public enum LinkType {
	/** Anchor link (#section) */
	ANCHOR("anchor"),

	/** Internal link (same domain) */
	INTERNAL("internal"),

	/** External link (different domain) */
	EXTERNAL("external"),

	/** Email link (mailto:) */
	EMAIL("email"),

	/** Phone link (tel:) */
	PHONE("phone"),

	/** Other link type */
	OTHER("other");

	private final String wireValue;

	LinkType(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this link type.
	 *
	 * @return wire value used in serialization (lowercase)
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse a LinkType from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (lowercase string)
	 * @return the corresponding LinkType
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	@JsonCreator
	public static LinkType fromWireValue(String wireValue) {
		for (LinkType type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown LinkType: " + wireValue);
	}
}

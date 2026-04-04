package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Structured data type classification.
 *
 * <p>
 * Classifies structured data as JSON-LD, microdata, or RDFa.
 *
 * @since 0.8.0
 */
public enum StructuredDataType {
	/** JSON-LD structured data */
	JSON_LD("json-ld"),

	/** Microdata */
	MICRODATA("microdata"),

	/** RDFa */
	RDFA("rdfa");

	private final String wireValue;

	StructuredDataType(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this structured data type.
	 *
	 * @return wire value used in serialization
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse a StructuredDataType from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (lowercase string or hyphenated)
	 * @return the corresponding StructuredDataType
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	@JsonCreator
	public static StructuredDataType fromWireValue(String wireValue) {
		for (StructuredDataType type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown StructuredDataType: " + wireValue);
	}
}

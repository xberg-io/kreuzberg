package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Semantic element type classification.
 *
 * <p>
 * Categorizes text content into semantic units for downstream processing.
 * Supports the element types commonly found in Unstructured documents.
 *
 * @since 4.1.0
 */
public enum ElementType {
	/** Document title */
	TITLE("title"),

	/** Main narrative text body */
	NARRATIVE_TEXT("narrative_text"),

	/** Section heading */
	HEADING("heading"),

	/** List item (bullet, numbered, etc.) */
	LIST_ITEM("list_item"),

	/** Table element */
	TABLE("table"),

	/** Image element */
	IMAGE("image"),

	/** Page break marker */
	PAGE_BREAK("page_break"),

	/** Code block */
	CODE_BLOCK("code_block"),

	/** Block quote */
	BLOCK_QUOTE("block_quote"),

	/** Footer text */
	FOOTER("footer"),

	/** Header text */
	HEADER("header");

	private final String wireValue;

	ElementType(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this element type.
	 *
	 * @return wire value used in serialization
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse an ElementType from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (snake_case string)
	 * @return the corresponding ElementType
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	public static ElementType fromWireValue(String wireValue) {
		for (ElementType type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown ElementType: " + wireValue);
	}
}

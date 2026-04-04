package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Types of inline elements in Djot.
 *
 * <p>
 * Represents formatting and semantic elements like text, emphasis, links,
 * images, code, math expressions, and other inline content.
 *
 * @since 0.8.0
 */
public enum InlineType {
	/** Plain text content */
	TEXT("text"),

	/** Strong/bold text */
	STRONG("strong"),

	/** Emphasized/italic text */
	EMPHASIS("emphasis"),

	/** Highlighted text */
	HIGHLIGHT("highlight"),

	/** Subscript text */
	SUBSCRIPT("subscript"),

	/** Superscript text */
	SUPERSCRIPT("superscript"),

	/** Inserted text */
	INSERT("insert"),

	/** Deleted/strikethrough text */
	DELETE("delete"),

	/** Inline code */
	CODE("code"),

	/** Hyperlink */
	LINK("link"),

	/** Image reference */
	IMAGE("image"),

	/** Generic span container */
	SPAN("span"),

	/** Inline math expression */
	MATH("math"),

	/** Raw inline content (HTML, LaTeX, etc.) */
	RAW_INLINE("raw_inline"),

	/** Footnote reference */
	FOOTNOTE_REF("footnote_ref"),

	/** Symbol or special character */
	SYMBOL("symbol");

	private final String wireValue;

	InlineType(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this inline type.
	 *
	 * @return wire value used in serialization (snake_case)
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse an InlineType from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (snake_case string)
	 * @return the corresponding InlineType
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	@JsonCreator
	public static InlineType fromWireValue(String wireValue) {
		for (InlineType type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown InlineType: " + wireValue);
	}
}

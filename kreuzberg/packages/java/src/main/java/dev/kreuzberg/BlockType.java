package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Types of block-level elements in Djot documents.
 *
 * <p>
 * Represents structural elements like headings, paragraphs, lists, code blocks,
 * blockquotes, and other container elements.
 *
 * @since 0.8.0
 */
public enum BlockType {
	/** Paragraph block */
	PARAGRAPH("paragraph"),

	/** Heading block (level 1-6) */
	HEADING("heading"),

	/** Block quote */
	BLOCKQUOTE("blockquote"),

	/** Code block */
	CODE_BLOCK("code_block"),

	/** List item */
	LIST_ITEM("list_item"),

	/** Ordered list */
	ORDERED_LIST("ordered_list"),

	/** Bullet list */
	BULLET_LIST("bullet_list"),

	/** Task list */
	TASK_LIST("task_list"),

	/** Definition list */
	DEFINITION_LIST("definition_list"),

	/** Definition term */
	DEFINITION_TERM("definition_term"),

	/** Definition description */
	DEFINITION_DESCRIPTION("definition_description"),

	/** Generic div container */
	DIV("div"),

	/** Section container */
	SECTION("section"),

	/** Thematic break (horizontal rule) */
	THEMATIC_BREAK("thematic_break"),

	/** Raw block (raw HTML, LaTeX, etc.) */
	RAW_BLOCK("raw_block"),

	/** Display math block */
	MATH_DISPLAY("math_display");

	private final String wireValue;

	BlockType(String wireValue) {
		this.wireValue = wireValue;
	}

	/**
	 * Get the wire format value for this block type.
	 *
	 * @return wire value used in serialization (snake_case)
	 */
	@JsonValue
	public String wireValue() {
		return wireValue;
	}

	/**
	 * Parse a BlockType from its wire value.
	 *
	 * @param wireValue
	 *            the wire format value (snake_case string)
	 * @return the corresponding BlockType
	 * @throws IllegalArgumentException
	 *             if the value is not recognized
	 */
	public static BlockType fromWireValue(String wireValue) {
		for (BlockType type : values()) {
			if (type.wireValue.equals(wireValue)) {
				return type;
			}
		}
		throw new IllegalArgumentException("Unknown BlockType: " + wireValue);
	}
}

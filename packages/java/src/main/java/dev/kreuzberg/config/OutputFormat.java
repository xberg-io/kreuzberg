package dev.kreuzberg.config;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Locale;

/**
 * Output format for extraction results.
 *
 * <p>
 * Controls the format of the content field in ExtractionResult. When set to
 * Markdown, Djot, or Html, the output will be formatted accordingly. Plain
 * returns the raw extracted text.
 *
 * @since 4.0.0
 */
public enum OutputFormat {
	/** Plain text content only (default). */
	PLAIN("plain"),
	/** Markdown format. */
	MARKDOWN("markdown"),
	/** Djot markup format. */
	DJOT("djot"),
	/** HTML format. */
	HTML("html"),
	/** JSON tree format with heading-driven sections. */
	JSON("json"),
	/** Structured JSON format with full OCR element metadata. */
	STRUCTURED("structured");

	private final String value;

	OutputFormat(String value) {
		this.value = value;
	}

	/**
	 * Gets the string representation for JSON serialization.
	 *
	 * @return the lowercase format name
	 */
	@JsonValue
	public String getValue() {
		return value;
	}

	/**
	 * Creates an OutputFormat from a string value (case-insensitive).
	 *
	 * @param value
	 *            the format name
	 * @return the OutputFormat enum value
	 * @throws IllegalArgumentException
	 *             if the value is invalid
	 */
	@JsonCreator
	public static OutputFormat fromValue(String value) {
		if (value == null) {
			return PLAIN;
		}
		String normalized = value.toLowerCase(Locale.ROOT);
		return switch (normalized) {
			case "plain", "text" -> PLAIN;
			case "markdown", "md" -> MARKDOWN;
			case "djot" -> DJOT;
			case "html" -> HTML;
			case "json" -> JSON;
			case "structured", "structured-ocr" -> STRUCTURED;
			default -> throw new IllegalArgumentException(
					"Invalid output format: '" + value + "'. Valid formats: plain, text, markdown, md, djot, html, json, structured");
		};
	}

	@Override
	public String toString() {
		return value;
	}
}

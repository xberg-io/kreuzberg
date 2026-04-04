package dev.kreuzberg;

/**
 * OCR element hierarchy levels.
 *
 * <p>
 * Represents the hierarchical level of an OCR-detected element in the document
 * structure.
 *
 * @since 4.4.0
 */
public final class OcrElementLevel {
	/** Word-level OCR element. */
	public static final String WORD = "word";

	/** Line-level OCR element. */
	public static final String LINE = "line";

	/** Block-level OCR element. */
	public static final String BLOCK = "block";

	/** Page-level OCR element. */
	public static final String PAGE = "page";

	private OcrElementLevel() {
	}
}

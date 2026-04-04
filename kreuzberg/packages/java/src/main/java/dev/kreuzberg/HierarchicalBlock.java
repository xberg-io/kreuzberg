package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * A text block with hierarchy level assignment.
 *
 * <p>
 * Represents a block of text with semantic heading information extracted from
 * font size clustering and hierarchical analysis.
 *
 * @param text
 *            the text content of this block
 * @param fontSize
 *            the font size of the text
 * @param level
 *            the hierarchy level (h1-h6 or body)
 * @param bbox
 *            bounding box as [left, top, right, bottom] in PDF units, or null
 */
public record HierarchicalBlock(@JsonProperty("text") String text, @JsonProperty("font_size") float fontSize,
		@JsonProperty("level") String level, @JsonProperty("bbox") float[] bbox) {
	@JsonCreator
	public HierarchicalBlock(@JsonProperty("text") String text, @JsonProperty("font_size") float fontSize,
			@JsonProperty("level") String level, @JsonProperty("bbox") float[] bbox) {
		this.text = text != null ? text : "";
		this.fontSize = fontSize;
		this.level = level != null ? level : "body";
		this.bbox = bbox;
	}
}

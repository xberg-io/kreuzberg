package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Tagged enumeration of node content types.
 *
 * <p>
 * Each node content variant carries only type-specific data. Since Java doesn't
 * have sealed unions, this class uses a discriminator field ({@code node_type})
 * with nullable fields for each variant.
 *
 * <p>
 * The actual variant is determined by the {@code node_type} field. Access
 * variant-specific data through the appropriate getter methods.
 *
 * @since 4.3.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class NodeContent {
	private final String nodeType;

	// Title variant
	private final String title;

	// Heading variant
	private final Integer headingLevel;
	private final String headingText;

	// Paragraph variant
	private final String paragraphText;

	// List variant
	private final Boolean ordered;

	// ListItem variant
	private final String listItemText;

	// Table variant
	private final TableGrid table;

	// Image variant
	private final String imageDescription;
	private final Integer imageIndex;
	private final String imageSrc;

	// Code variant
	private final String codeText;
	private final String codeLanguage;

	// Quote - no extra fields

	// Formula variant
	private final String formulaText;

	// Footnote variant
	private final String footnoteText;

	// Group variant
	private final String groupLabel;
	private final Integer groupHeadingLevel;
	private final String groupHeadingText;

	// PageBreak - no extra fields

	/**
	 * Create a new NodeContent with full initialization.
	 *
	 * <p>
	 * This constructor accepts all possible fields. Only fields relevant to the
	 * {@code nodeType} should be non-null.
	 *
	 * @param nodeType
	 *            the content type discriminator (must not be null)
	 * @param text
	 *            text content, mapped to the appropriate field based on nodeType
	 * @param headingLevel
	 *            heading level 1-6 (for Heading nodes)
	 * @param ordered
	 *            whether list is ordered (for List nodes)
	 * @param table
	 *            table grid (for Table nodes)
	 * @param imageDescription
	 *            image alt text (for Image nodes)
	 * @param imageIndex
	 *            image index (for Image nodes)
	 * @param imageSrc
	 *            source URL or path (for Image nodes)
	 * @param codeLanguage
	 *            programming language (for Code nodes)
	 * @param groupLabel
	 *            group label (for Group nodes)
	 * @param groupHeadingLevel
	 *            group heading level (for Group nodes)
	 * @param groupHeadingText
	 *            group heading text (for Group nodes)
	 */
	@JsonCreator
	public NodeContent(@JsonProperty("node_type") String nodeType, @JsonProperty("text") String text,
			@JsonProperty("level") Integer headingLevel, @JsonProperty("ordered") Boolean ordered,
			@JsonProperty("grid") TableGrid table, @JsonProperty("description") String imageDescription,
			@JsonProperty("image_index") Integer imageIndex, @JsonProperty("src") String imageSrc,
			@JsonProperty("language") String codeLanguage,
			@JsonProperty("label") String groupLabel, @JsonProperty("heading_level") Integer groupHeadingLevel,
			@JsonProperty("heading_text") String groupHeadingText) {
		this.nodeType = Objects.requireNonNull(nodeType, "nodeType must not be null");
		this.title = "title".equals(nodeType) ? text : null;
		this.headingLevel = headingLevel;
		this.headingText = "heading".equals(nodeType) ? text : null;
		this.paragraphText = "paragraph".equals(nodeType) ? text : null;
		this.ordered = ordered;
		this.listItemText = "list_item".equals(nodeType) ? text : null;
		this.table = table;
		this.imageDescription = imageDescription;
		this.imageIndex = imageIndex;
		this.imageSrc = imageSrc;
		this.codeText = "code".equals(nodeType) ? text : null;
		this.codeLanguage = codeLanguage;
		this.formulaText = "formula".equals(nodeType) ? text : null;
		this.footnoteText = "footnote".equals(nodeType) ? text : null;
		this.groupLabel = groupLabel;
		this.groupHeadingLevel = groupHeadingLevel;
		this.groupHeadingText = groupHeadingText;
	}

	/**
	 * Get the node type discriminator.
	 *
	 * <p>
	 * Possible values: "title", "heading", "paragraph", "list", "list_item",
	 * "table", "image", "code", "quote", "formula", "footnote", "group",
	 * "page_break".
	 *
	 * @return node type string (never null)
	 */
	@JsonProperty("node_type")
	public String getNodeType() {
		return nodeType;
	}

	/**
	 * Get the title text if this is a Title node.
	 *
	 * @return title text, or empty if not a Title node
	 */
	public Optional<String> getTitle() {
		return "title".equals(nodeType) ? Optional.ofNullable(title) : Optional.empty();
	}

	/**
	 * Get the heading level if this is a Heading node.
	 *
	 * @return heading level (1-6), or empty if not a Heading node
	 */
	public Optional<Integer> getHeadingLevel() {
		return "heading".equals(nodeType) ? Optional.ofNullable(headingLevel) : Optional.empty();
	}

	/**
	 * Get the heading text if this is a Heading node.
	 *
	 * @return heading text, or empty if not a Heading node
	 */
	@JsonProperty("text")
	public Optional<String> getHeadingText() {
		return "heading".equals(nodeType) ? Optional.ofNullable(headingText) : Optional.empty();
	}

	/**
	 * Get the paragraph text if this is a Paragraph node.
	 *
	 * @return paragraph text, or empty if not a Paragraph node
	 */
	public Optional<String> getParagraphText() {
		return "paragraph".equals(nodeType) ? Optional.ofNullable(paragraphText) : Optional.empty();
	}

	/**
	 * Check if list is ordered (true) or unordered (false).
	 *
	 * @return true for ordered lists, false for unordered, empty if not a List node
	 */
	public Optional<Boolean> isOrdered() {
		return "list".equals(nodeType) ? Optional.ofNullable(ordered) : Optional.empty();
	}

	/**
	 * Get the list item text if this is a ListItem node.
	 *
	 * @return list item text, or empty if not a ListItem node
	 */
	public Optional<String> getListItemText() {
		return "list_item".equals(nodeType) ? Optional.ofNullable(listItemText) : Optional.empty();
	}

	/**
	 * Get the table grid if this is a Table node.
	 *
	 * @return table grid, or empty if not a Table node
	 */
	public Optional<TableGrid> getTableGrid() {
		return "table".equals(nodeType) ? Optional.ofNullable(table) : Optional.empty();
	}

	/**
	 * Get the image description if this is an Image node.
	 *
	 * @return alt text, or empty if not an Image node
	 */
	public Optional<String> getImageDescription() {
		return "image".equals(nodeType) ? Optional.ofNullable(imageDescription) : Optional.empty();
	}

	/**
	 * Get the image index if this is an Image node.
	 *
	 * @return image index, or empty if not an Image node
	 */
	public Optional<Integer> getImageIndex() {
		return "image".equals(nodeType) ? Optional.ofNullable(imageIndex) : Optional.empty();
	}

	/**
	 * Get the image source URL or path if this is an Image node.
	 *
	 * @return image src, or empty if not an Image node
	 */
	public Optional<String> getImageSrc() {
		return "image".equals(nodeType) ? Optional.ofNullable(imageSrc) : Optional.empty();
	}

	/**
	 * Get the code text if this is a Code node.
	 *
	 * @return code content, or empty if not a Code node
	 */
	public Optional<String> getCodeText() {
		return "code".equals(nodeType) ? Optional.ofNullable(codeText) : Optional.empty();
	}

	/**
	 * Get the programming language if this is a Code node.
	 *
	 * @return language name, or empty if not a Code node
	 */
	public Optional<String> getCodeLanguage() {
		return "code".equals(nodeType) ? Optional.ofNullable(codeLanguage) : Optional.empty();
	}

	/**
	 * Get the formula expression if this is a Formula node.
	 *
	 * @return formula text, or empty if not a Formula node
	 */
	public Optional<String> getFormulaText() {
		return "formula".equals(nodeType) ? Optional.ofNullable(formulaText) : Optional.empty();
	}

	/**
	 * Get the footnote text if this is a Footnote node.
	 *
	 * @return footnote text, or empty if not a Footnote node
	 */
	public Optional<String> getFootnoteText() {
		return "footnote".equals(nodeType) ? Optional.ofNullable(footnoteText) : Optional.empty();
	}

	/**
	 * Get the group label if this is a Group node.
	 *
	 * @return group label, or empty if not a Group node
	 */
	public Optional<String> getGroupLabel() {
		return "group".equals(nodeType) ? Optional.ofNullable(groupLabel) : Optional.empty();
	}

	/**
	 * Get the group heading level if this is a Group node.
	 *
	 * @return heading level, or empty if not a Group node
	 */
	public Optional<Integer> getGroupHeadingLevel() {
		return "group".equals(nodeType) ? Optional.ofNullable(groupHeadingLevel) : Optional.empty();
	}

	/**
	 * Get the group heading text if this is a Group node.
	 *
	 * @return heading text, or empty if not a Group node
	 */
	public Optional<String> getGroupHeadingText() {
		return "group".equals(nodeType) ? Optional.ofNullable(groupHeadingText) : Optional.empty();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof NodeContent)) {
			return false;
		}
		NodeContent other = (NodeContent) obj;
		return Objects.equals(nodeType, other.nodeType) && Objects.equals(title, other.title)
				&& Objects.equals(headingLevel, other.headingLevel) && Objects.equals(headingText, other.headingText)
				&& Objects.equals(paragraphText, other.paragraphText) && Objects.equals(ordered, other.ordered)
				&& Objects.equals(listItemText, other.listItemText) && Objects.equals(table, other.table)
				&& Objects.equals(imageDescription, other.imageDescription)
				&& Objects.equals(imageIndex, other.imageIndex) && Objects.equals(imageSrc, other.imageSrc)
				&& Objects.equals(codeText, other.codeText)
				&& Objects.equals(codeLanguage, other.codeLanguage) && Objects.equals(formulaText, other.formulaText)
				&& Objects.equals(footnoteText, other.footnoteText) && Objects.equals(groupLabel, other.groupLabel)
				&& Objects.equals(groupHeadingLevel, other.groupHeadingLevel)
				&& Objects.equals(groupHeadingText, other.groupHeadingText);
	}

	@Override
	public int hashCode() {
		return Objects.hash(nodeType, title, headingLevel, headingText, paragraphText, ordered, listItemText, table,
				imageDescription, imageIndex, imageSrc, codeText, codeLanguage, formulaText, footnoteText, groupLabel,
				groupHeadingLevel, groupHeadingText);
	}

	@Override
	public String toString() {
		return "NodeContent{" + "nodeType='" + nodeType + '\'' + '}';
	}
}

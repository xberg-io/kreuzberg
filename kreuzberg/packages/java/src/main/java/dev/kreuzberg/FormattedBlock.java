package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * Block-level element in a Djot document.
 *
 * <p>
 * Represents structural elements like headings, paragraphs, lists, code blocks,
 * blockquotes, and other container elements. Blocks can contain inline content
 * and nested child blocks.
 *
 * @since 0.8.0
 */
public final class FormattedBlock {
	private final BlockType blockType;
	private final Optional<Integer> level;
	@JsonDeserialize(contentAs = InlineElement.class)
	private final List<InlineElement> inlineContent;
	private final Optional<Attributes> attributes;
	private final Optional<String> language;
	private final Optional<String> code;
	@JsonDeserialize(contentAs = FormattedBlock.class)
	private final List<FormattedBlock> children;

	@JsonCreator
	public FormattedBlock(@JsonProperty("block_type") BlockType blockType,
			@JsonProperty("level") Optional<Integer> level,
			@JsonProperty("inline_content") List<InlineElement> inlineContent,
			@JsonProperty("attributes") Optional<Attributes> attributes,
			@JsonProperty("language") Optional<String> language, @JsonProperty("code") Optional<String> code,
			@JsonProperty("children") List<FormattedBlock> children) {
		this.blockType = Objects.requireNonNull(blockType, "blockType must not be null");
		this.level = level != null ? level : Optional.empty();
		this.inlineContent = Collections
				.unmodifiableList(inlineContent != null ? new ArrayList<>(inlineContent) : new ArrayList<>());
		this.attributes = attributes != null ? attributes : Optional.empty();
		this.language = language != null ? language : Optional.empty();
		this.code = code != null ? code : Optional.empty();
		this.children = Collections.unmodifiableList(children != null ? new ArrayList<>(children) : new ArrayList<>());
	}

	/**
	 * Creates a new FormattedBlock with required fields only.
	 *
	 * @param blockType
	 *            the type of block element
	 * @param inlineContent
	 *            inline content within the block
	 * @return a new FormattedBlock instance
	 */
	public static FormattedBlock of(BlockType blockType, List<InlineElement> inlineContent) {
		return new FormattedBlock(blockType, Optional.empty(), inlineContent, Optional.empty(), Optional.empty(),
				Optional.empty(), Collections.emptyList());
	}

	/**
	 * Creates a new FormattedBlock with all fields.
	 *
	 * @param blockType
	 *            the type of block element
	 * @param level
	 *            heading level (1-6) or nesting level
	 * @param inlineContent
	 *            inline content within the block
	 * @param attributes
	 *            optional element attributes
	 * @param language
	 *            optional language identifier for code blocks
	 * @param code
	 *            optional raw code content for code blocks
	 * @param children
	 *            nested blocks for containers
	 * @return a new FormattedBlock instance
	 */
	public static FormattedBlock of(BlockType blockType, Optional<Integer> level, List<InlineElement> inlineContent,
			Optional<Attributes> attributes, Optional<String> language, Optional<String> code,
			List<FormattedBlock> children) {
		return new FormattedBlock(blockType, level, inlineContent, attributes, language, code, children);
	}

	/**
	 * Get the type of block element.
	 *
	 * @return block type
	 */
	public BlockType getBlockType() {
		return blockType;
	}

	/**
	 * Get the heading level (1-6) for headings, or nesting level for lists.
	 *
	 * @return optional heading/nesting level
	 */
	public Optional<Integer> getLevel() {
		return level;
	}

	/**
	 * Get the inline content within this block.
	 *
	 * @return unmodifiable list of inline elements (never null)
	 */
	public List<InlineElement> getInlineContent() {
		return inlineContent;
	}

	/**
	 * Get the optional element attributes.
	 *
	 * @return optional attributes
	 */
	public Optional<Attributes> getAttributes() {
		return attributes;
	}

	/**
	 * Get the optional language identifier for code blocks.
	 *
	 * @return optional language identifier (e.g., "python", "rust", "javascript")
	 */
	public Optional<String> getLanguage() {
		return language;
	}

	/**
	 * Get the optional raw code content for code blocks.
	 *
	 * @return optional code content
	 */
	public Optional<String> getCode() {
		return code;
	}

	/**
	 * Get the nested blocks for containers (blockquotes, list items, divs).
	 *
	 * @return unmodifiable list of child blocks (never null, may be empty)
	 */
	public List<FormattedBlock> getChildren() {
		return children;
	}

	/**
	 * Check if this block has child blocks.
	 *
	 * @return true if this block has children
	 */
	public boolean hasChildren() {
		return !children.isEmpty();
	}

	/**
	 * Get the plain text content of this block and its children.
	 *
	 * <p>
	 * Recursively extracts all text from inline content and child blocks.
	 *
	 * @return plain text representation
	 */
	public String getPlainText() {
		StringBuilder sb = new StringBuilder();

		for (InlineElement inline : inlineContent) {
			sb.append(inline.getContent());
		}

		for (FormattedBlock child : children) {
			if (sb.length() > 0) {
				sb.append("\n");
			}
			sb.append(child.getPlainText());
		}

		return sb.toString();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof FormattedBlock)) {
			return false;
		}
		FormattedBlock other = (FormattedBlock) obj;
		return Objects.equals(blockType, other.blockType) && Objects.equals(level, other.level)
				&& Objects.equals(inlineContent, other.inlineContent) && Objects.equals(attributes, other.attributes)
				&& Objects.equals(language, other.language) && Objects.equals(code, other.code)
				&& Objects.equals(children, other.children);
	}

	@Override
	public int hashCode() {
		return Objects.hash(blockType, level, inlineContent, attributes, language, code, children);
	}

	@Override
	public String toString() {
		return "FormattedBlock{" + "blockType=" + blockType + ", level=" + level + ", inlineContent="
				+ inlineContent.size() + ", attributes=" + attributes + ", language=" + language + ", code="
				+ (code.isPresent() ? code.get().length() : "none") + ", children=" + children.size() + '}';
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Comprehensive Djot document structure with semantic preservation.
 *
 * <p>
 * Captures the full richness of Djot markup, including:
 * <ul>
 * <li>Block-level structures (headings, lists, blockquotes, code blocks,
 * etc.)</li>
 * <li>Inline formatting (emphasis, strong, highlight, subscript, superscript,
 * etc.)</li>
 * <li>Attributes (classes, IDs, key-value pairs)</li>
 * <li>Links, images, footnotes</li>
 * <li>Math expressions (inline and display)</li>
 * <li>Tables with full structure</li>
 * </ul>
 *
 * <p>
 * Available when the Djot feature is enabled.
 *
 * @since 0.8.0
 */
public final class DjotContent {
	private final String plainText;
	@JsonDeserialize(contentAs = FormattedBlock.class)
	private final List<FormattedBlock> blocks;
	private final Metadata metadata;
	@JsonDeserialize(contentAs = Table.class)
	private final List<Table> tables;
	@JsonDeserialize(contentAs = DjotImage.class)
	private final List<DjotImage> images;
	@JsonDeserialize(contentAs = DjotLink.class)
	private final List<DjotLink> links;
	@JsonDeserialize(contentAs = Footnote.class)
	private final List<Footnote> footnotes;
	@JsonDeserialize(contentAs = AttributeEntry.class)
	private final List<AttributeEntry> attributes;

	@JsonCreator
	public DjotContent(@JsonProperty("plain_text") String plainText,
			@JsonProperty("blocks") List<FormattedBlock> blocks, @JsonProperty("metadata") Metadata metadata,
			@JsonProperty("tables") List<Table> tables, @JsonProperty("images") List<DjotImage> images,
			@JsonProperty("links") List<DjotLink> links, @JsonProperty("footnotes") List<Footnote> footnotes,
			@JsonProperty("attributes") List<AttributeEntry> attributes) {
		this.plainText = Objects.requireNonNull(plainText, "plainText must not be null");
		this.blocks = Collections.unmodifiableList(blocks != null ? new ArrayList<>(blocks) : new ArrayList<>());
		this.metadata = metadata != null ? metadata : Metadata.empty();
		this.tables = Collections.unmodifiableList(tables != null ? new ArrayList<>(tables) : new ArrayList<>());
		this.images = Collections.unmodifiableList(images != null ? new ArrayList<>(images) : new ArrayList<>());
		this.links = Collections.unmodifiableList(links != null ? new ArrayList<>(links) : new ArrayList<>());
		this.footnotes = Collections
				.unmodifiableList(footnotes != null ? new ArrayList<>(footnotes) : new ArrayList<>());
		this.attributes = Collections
				.unmodifiableList(attributes != null ? new ArrayList<>(attributes) : new ArrayList<>());
	}

	/**
	 * Get the plain text representation of the document.
	 *
	 * @return plain text
	 */
	public String getPlainText() {
		return plainText;
	}

	/**
	 * Get the block-level content of the document.
	 *
	 * @return unmodifiable list of blocks (never null)
	 */
	public List<FormattedBlock> getBlocks() {
		return blocks;
	}

	/**
	 * Get the document metadata.
	 *
	 * @return metadata (never null)
	 */
	public Metadata getMetadata() {
		return metadata;
	}

	/**
	 * Get the extracted tables.
	 *
	 * @return unmodifiable list of tables (never null)
	 */
	public List<Table> getTables() {
		return tables;
	}

	/**
	 * Get the extracted images.
	 *
	 * @return unmodifiable list of images (never null)
	 */
	public List<DjotImage> getImages() {
		return images;
	}

	/**
	 * Get the extracted links.
	 *
	 * @return unmodifiable list of links (never null)
	 */
	public List<DjotLink> getLinks() {
		return links;
	}

	/**
	 * Get the footnote definitions.
	 *
	 * @return unmodifiable list of footnotes (never null)
	 */
	public List<Footnote> getFootnotes() {
		return footnotes;
	}

	/**
	 * Get the element attributes mapped by identifier.
	 *
	 * @return unmodifiable list of attribute entries (never null)
	 */
	public List<AttributeEntry> getAttributes() {
		return attributes;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof DjotContent)) {
			return false;
		}
		DjotContent other = (DjotContent) obj;
		return Objects.equals(plainText, other.plainText) && Objects.equals(blocks, other.blocks)
				&& Objects.equals(metadata, other.metadata) && Objects.equals(tables, other.tables)
				&& Objects.equals(images, other.images) && Objects.equals(links, other.links)
				&& Objects.equals(footnotes, other.footnotes) && Objects.equals(attributes, other.attributes);
	}

	@Override
	public int hashCode() {
		return Objects.hash(plainText, blocks, metadata, tables, images, links, footnotes, attributes);
	}

	@Override
	public String toString() {
		return "DjotContent{" + "plainTextLength=" + plainText.length() + ", blocks=" + blocks.size() + ", tables="
				+ tables.size() + ", images=" + images.size() + ", links=" + links.size() + ", footnotes="
				+ footnotes.size() + ", attributes=" + attributes.size() + '}';
	}

	/**
	 * Represents an element attribute entry (String key to Attributes mapping).
	 */
	public static final class AttributeEntry {
		private final String key;
		private final Attributes attributes;

		@JsonCreator
		public AttributeEntry(@JsonProperty("0") String key, @JsonProperty("1") Attributes attributes) {
			this.key = Objects.requireNonNull(key, "key must not be null");
			this.attributes = Objects.requireNonNull(attributes, "attributes must not be null");
		}

		/**
		 * Creates a new attribute entry.
		 *
		 * @param key
		 *            the element identifier
		 * @param attributes
		 *            the attributes for this element
		 * @return a new AttributeEntry instance
		 */
		public static AttributeEntry of(String key, Attributes attributes) {
			return new AttributeEntry(key, attributes);
		}

		/**
		 * Get the element identifier key.
		 *
		 * @return element key
		 */
		public String getKey() {
			return key;
		}

		/**
		 * Get the attributes for this element.
		 *
		 * @return attributes
		 */
		public Attributes getAttributes() {
			return attributes;
		}

		@Override
		public boolean equals(Object obj) {
			if (this == obj) {
				return true;
			}
			if (!(obj instanceof AttributeEntry)) {
				return false;
			}
			AttributeEntry other = (AttributeEntry) obj;
			return Objects.equals(key, other.key) && Objects.equals(attributes, other.attributes);
		}

		@Override
		public int hashCode() {
			return Objects.hash(key, attributes);
		}

		@Override
		public String toString() {
			return "AttributeEntry{" + "key='" + key + '\'' + ", attributes=" + attributes + '}';
		}
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * Semantic element extracted from document.
 *
 * <p>
 * Represents a logical unit of content with semantic classification, unique
 * identifier, and metadata for tracking origin and position. Supports the
 * element types commonly found in Unstructured documents.
 *
 * <p>
 * When extraction is configured with {@code output_format="element_based"}, the
 * extraction result includes a list of these semantic elements instead of flat
 * text content.
 *
 * @since 4.1.0
 */
public final class Element {
	private final String elementId;
	private final ElementType elementType;
	private final String text;
	private final ElementMetadata metadata;

	/**
	 * Create a new semantic element.
	 *
	 * @param elementId
	 *            unique element identifier (deterministic hash-based ID)
	 * @param elementType
	 *            semantic type classification
	 * @param text
	 *            text content of the element
	 * @param metadata
	 *            element metadata including page number, coordinates, etc.
	 */
	@JsonCreator
	public Element(@JsonProperty("element_id") String elementId, @JsonProperty("element_type") ElementType elementType,
			@JsonProperty("text") String text, @JsonProperty("metadata") ElementMetadata metadata) {
		this.elementId = Objects.requireNonNull(elementId, "elementId must not be null");
		this.elementType = Objects.requireNonNull(elementType, "elementType must not be null");
		this.text = Objects.requireNonNull(text, "text must not be null");
		this.metadata = Objects.requireNonNull(metadata, "metadata must not be null");
	}

	/**
	 * Get the unique element identifier.
	 *
	 * <p>
	 * This is a deterministic hash-based ID that remains stable for the same
	 * element across multiple extractions.
	 *
	 * @return element ID string
	 */
	@JsonProperty("element_id")
	public String getElementId() {
		return elementId;
	}

	/**
	 * Get the semantic type of this element.
	 *
	 * @return element type enumeration
	 */
	@JsonProperty("element_type")
	public ElementType getElementType() {
		return elementType;
	}

	/**
	 * Get the text content of the element.
	 *
	 * @return text content string
	 */
	@JsonProperty("text")
	public String getText() {
		return text;
	}

	/**
	 * Get the metadata about the element.
	 *
	 * <p>
	 * Includes information about location (page, coordinates), origin (filename),
	 * and position in the element sequence.
	 *
	 * @return element metadata
	 */
	@JsonProperty("metadata")
	public ElementMetadata getMetadata() {
		return metadata;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof Element)) {
			return false;
		}
		Element other = (Element) obj;
		return Objects.equals(elementId, other.elementId) && elementType == other.elementType
				&& Objects.equals(text, other.text) && Objects.equals(metadata, other.metadata);
	}

	@Override
	public int hashCode() {
		return Objects.hash(elementId, elementType, text, metadata);
	}

	@Override
	public String toString() {
		return "Element{" + "elementId='" + elementId + '\'' + ", elementType=" + elementType + ", textLength="
				+ text.length() + ", metadata=" + metadata + '}';
	}
}

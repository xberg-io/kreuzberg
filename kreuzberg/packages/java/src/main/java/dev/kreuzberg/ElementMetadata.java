package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Metadata for a semantic element.
 *
 * <p>
 * Provides contextual information about an element's location and origin,
 * including page number, source filename, bounding box coordinates, and custom
 * metadata fields.
 *
 * @since 4.1.0
 */
public final class ElementMetadata {
	private final Integer pageNumber;
	private final String filename;
	private final BoundingBox coordinates;
	private final Integer elementIndex;
	private final Map<String, String> additional;

	/**
	 * Create new element metadata.
	 *
	 * @param pageNumber
	 *            page number (1-indexed), or null if not available
	 * @param filename
	 *            source filename or document name, or null if not available
	 * @param coordinates
	 *            bounding box coordinates, or null if not available
	 * @param elementIndex
	 *            position index in element sequence, or null if not available
	 * @param additional
	 *            additional custom metadata fields (may be null or empty)
	 */
	@JsonCreator
	public ElementMetadata(@JsonProperty("page_number") Integer pageNumber, @JsonProperty("filename") String filename,
			@JsonProperty("coordinates") BoundingBox coordinates, @JsonProperty("element_index") Integer elementIndex,
			@JsonProperty("additional") Map<String, String> additional) {
		if (pageNumber != null && pageNumber < 1) {
			throw new IllegalArgumentException("pageNumber must be positive");
		}
		if (elementIndex != null && elementIndex < 0) {
			throw new IllegalArgumentException("elementIndex must be non-negative");
		}
		this.pageNumber = pageNumber;
		this.filename = filename;
		this.coordinates = coordinates;
		this.elementIndex = elementIndex;
		if (additional != null && !additional.isEmpty()) {
			this.additional = Collections.unmodifiableMap(new HashMap<>(additional));
		} else {
			this.additional = Collections.emptyMap();
		}
	}

	/**
	 * Get the page number where this element appears (1-indexed, optional).
	 *
	 * @return page number, or empty if not available
	 */
	@JsonProperty("page_number")
	public Optional<Integer> getPageNumber() {
		return Optional.ofNullable(pageNumber);
	}

	/**
	 * Get the source filename or document name (optional).
	 *
	 * @return filename, or empty if not available
	 */
	@JsonProperty("filename")
	public Optional<String> getFilename() {
		return Optional.ofNullable(filename);
	}

	/**
	 * Get the bounding box coordinates for spatial positioning (optional).
	 *
	 * @return bounding box, or empty if not available
	 */
	@JsonProperty("coordinates")
	public Optional<BoundingBox> getCoordinates() {
		return Optional.ofNullable(coordinates);
	}

	/**
	 * Get the element's index in the sequence of extracted elements (optional).
	 *
	 * @return zero-based element index, or empty if not available
	 */
	@JsonProperty("element_index")
	public Optional<Integer> getElementIndex() {
		return Optional.ofNullable(elementIndex);
	}

	/**
	 * Get additional custom metadata fields.
	 *
	 * @return unmodifiable map of additional metadata (never null, but may be
	 *         empty)
	 */
	@JsonProperty("additional")
	public Map<String, String> getAdditional() {
		return additional;
	}

	/**
	 * Get a custom metadata field by name.
	 *
	 * @param key
	 *            the metadata field name
	 * @return the field value, or empty if not present
	 */
	public Optional<String> getAdditionalField(String key) {
		return Optional.ofNullable(additional.get(key));
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof ElementMetadata)) {
			return false;
		}
		ElementMetadata other = (ElementMetadata) obj;
		return Objects.equals(pageNumber, other.pageNumber) && Objects.equals(filename, other.filename)
				&& Objects.equals(coordinates, other.coordinates) && Objects.equals(elementIndex, other.elementIndex)
				&& Objects.equals(additional, other.additional);
	}

	@Override
	public int hashCode() {
		return Objects.hash(pageNumber, filename, coordinates, elementIndex, additional);
	}

	@Override
	public String toString() {
		return "ElementMetadata{" + "pageNumber=" + pageNumber + ", filename='" + filename + '\'' + ", coordinates="
				+ coordinates + ", elementIndex=" + elementIndex + ", additionalFields=" + additional.size() + '}';
	}
}

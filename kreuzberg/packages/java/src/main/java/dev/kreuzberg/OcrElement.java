package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.Map;
import java.util.Objects;

/**
 * OCR-extracted element with text, geometry, confidence, and metadata.
 *
 * <p>
 * Represents a single OCR-detected text element with its bounding geometry,
 * confidence scores, and hierarchical information.
 *
 * @since 4.4.0
 */
public final class OcrElement {
	private final String text;
	private final OcrBoundingGeometry geometry;
	private final OcrConfidence confidence;
	private final String level;
	private final OcrRotation rotation;
	private final Integer pageNumber;
	private final String parentId;
	private final Map<String, Object> backendMetadata;

	@JsonCreator
	public OcrElement(@JsonProperty("text") String text, @JsonProperty("geometry") OcrBoundingGeometry geometry,
			@JsonProperty("confidence") OcrConfidence confidence, @JsonProperty("level") String level,
			@JsonProperty("rotation") OcrRotation rotation, @JsonProperty("page_number") Integer pageNumber,
			@JsonProperty("parent_id") String parentId,
			@JsonProperty("backend_metadata") Map<String, Object> backendMetadata) {
		this.text = Objects.requireNonNull(text, "text must not be null");
		this.geometry = geometry;
		this.confidence = confidence;
		this.level = level;
		this.rotation = rotation;
		this.pageNumber = pageNumber;
		this.parentId = parentId;
		this.backendMetadata = backendMetadata != null
				? Collections.unmodifiableMap(backendMetadata)
				: Collections.emptyMap();
	}

	@JsonProperty("text")
	public String getText() {
		return text;
	}

	@JsonProperty("geometry")
	public OcrBoundingGeometry getGeometry() {
		return geometry;
	}

	@JsonProperty("confidence")
	public OcrConfidence getConfidence() {
		return confidence;
	}

	@JsonProperty("level")
	public String getLevel() {
		return level;
	}

	@JsonProperty("rotation")
	public OcrRotation getRotation() {
		return rotation;
	}

	@JsonProperty("page_number")
	public Integer getPageNumber() {
		return pageNumber;
	}

	@JsonProperty("parent_id")
	public String getParentId() {
		return parentId;
	}

	@JsonProperty("backend_metadata")
	public Map<String, Object> getBackendMetadata() {
		return backendMetadata;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof OcrElement)) {
			return false;
		}
		OcrElement other = (OcrElement) obj;
		return Objects.equals(text, other.text) && Objects.equals(geometry, other.geometry)
				&& Objects.equals(confidence, other.confidence) && Objects.equals(level, other.level)
				&& Objects.equals(rotation, other.rotation) && Objects.equals(pageNumber, other.pageNumber)
				&& Objects.equals(parentId, other.parentId) && Objects.equals(backendMetadata, other.backendMetadata);
	}

	@Override
	public int hashCode() {
		return Objects.hash(text, geometry, confidence, level, rotation, pageNumber, parentId, backendMetadata);
	}

	@Override
	public String toString() {
		return "OcrElement{" + "text='" + text + '\'' + ", level='" + level + '\'' + ", pageNumber=" + pageNumber
				+ ", hasGeometry=" + (geometry != null) + ", hasConfidence=" + (confidence != null) + ", hasRotation="
				+ (rotation != null) + '}';
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Bounding geometry for an OCR element.
 *
 * <p>
 * Represents the spatial bounds of an OCR-detected element, supporting both
 * rectangular bounds (left, top, width, height) and arbitrary polygon points.
 *
 * @since 4.4.0
 */
public final class OcrBoundingGeometry {
	private final String type;
	private final Double left;
	private final Double top;
	private final Double width;
	private final Double height;
	private final List<List<Double>> points;

	@JsonCreator
	public OcrBoundingGeometry(@JsonProperty("type") String type, @JsonProperty("left") Double left,
			@JsonProperty("top") Double top, @JsonProperty("width") Double width, @JsonProperty("height") Double height,
			@JsonProperty("points") List<List<Double>> points) {
		this.type = type;
		this.left = left;
		this.top = top;
		this.width = width;
		this.height = height;
		this.points = points != null ? Collections.unmodifiableList(points) : Collections.emptyList();
	}

	@JsonProperty("type")
	public String getType() {
		return type;
	}

	@JsonProperty("left")
	public Double getLeft() {
		return left;
	}

	@JsonProperty("top")
	public Double getTop() {
		return top;
	}

	@JsonProperty("width")
	public Double getWidth() {
		return width;
	}

	@JsonProperty("height")
	public Double getHeight() {
		return height;
	}

	@JsonProperty("points")
	public List<List<Double>> getPoints() {
		return points;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof OcrBoundingGeometry)) {
			return false;
		}
		OcrBoundingGeometry other = (OcrBoundingGeometry) obj;
		return Objects.equals(type, other.type) && Objects.equals(left, other.left) && Objects.equals(top, other.top)
				&& Objects.equals(width, other.width) && Objects.equals(height, other.height)
				&& Objects.equals(points, other.points);
	}

	@Override
	public int hashCode() {
		return Objects.hash(type, left, top, width, height, points);
	}

	@Override
	public String toString() {
		return "OcrBoundingGeometry{" + "type='" + type + '\'' + ", left=" + left + ", top=" + top + ", width=" + width
				+ ", height=" + height + ", pointsCount=" + points.size() + '}';
	}
}

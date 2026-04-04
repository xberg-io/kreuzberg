package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * Bounding box coordinates for element positioning.
 *
 * <p>
 * Represents the spatial location of an element on a page using normalized or
 * pixel coordinates. All values are inclusive at the minimum corner and
 * exclusive at the maximum corner (standard rectangle convention).
 *
 * @since 4.1.0
 */
public final class BoundingBox {
	private final double x0;
	private final double y0;
	private final double x1;
	private final double y1;

	/**
	 * Create a new bounding box with the given coordinates.
	 *
	 * @param x0
	 *            left x-coordinate
	 * @param y0
	 *            bottom y-coordinate
	 * @param x1
	 *            right x-coordinate (must be >= x0)
	 * @param y1
	 *            top y-coordinate (must be >= y0)
	 */
	@JsonCreator
	public BoundingBox(@JsonProperty("x0") double x0, @JsonProperty("y0") double y0, @JsonProperty("x1") double x1,
			@JsonProperty("y1") double y1) {
		if (x1 < x0) {
			throw new IllegalArgumentException("x1 must be >= x0");
		}
		if (y1 < y0) {
			throw new IllegalArgumentException("y1 must be >= y0");
		}
		this.x0 = x0;
		this.y0 = y0;
		this.x1 = x1;
		this.y1 = y1;
	}

	/**
	 * Get the left x-coordinate.
	 *
	 * @return left x-coordinate
	 */
	@JsonProperty("x0")
	public double getX0() {
		return x0;
	}

	/**
	 * Get the bottom y-coordinate.
	 *
	 * @return bottom y-coordinate
	 */
	@JsonProperty("y0")
	public double getY0() {
		return y0;
	}

	/**
	 * Get the right x-coordinate.
	 *
	 * @return right x-coordinate
	 */
	@JsonProperty("x1")
	public double getX1() {
		return x1;
	}

	/**
	 * Get the top y-coordinate.
	 *
	 * @return top y-coordinate
	 */
	@JsonProperty("y1")
	public double getY1() {
		return y1;
	}

	/**
	 * Calculate the width of the bounding box.
	 *
	 * @return width (x1 - x0)
	 */
	public double getWidth() {
		return x1 - x0;
	}

	/**
	 * Calculate the height of the bounding box.
	 *
	 * @return height (y1 - y0)
	 */
	public double getHeight() {
		return y1 - y0;
	}

	/**
	 * Calculate the area of the bounding box.
	 *
	 * @return area (width * height)
	 */
	public double getArea() {
		return getWidth() * getHeight();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof BoundingBox)) {
			return false;
		}
		BoundingBox other = (BoundingBox) obj;
		return Double.doubleToLongBits(x0) == Double.doubleToLongBits(other.x0)
				&& Double.doubleToLongBits(y0) == Double.doubleToLongBits(other.y0)
				&& Double.doubleToLongBits(x1) == Double.doubleToLongBits(other.x1)
				&& Double.doubleToLongBits(y1) == Double.doubleToLongBits(other.y1);
	}

	@Override
	public int hashCode() {
		return Objects.hash(Double.doubleToLongBits(x0), Double.doubleToLongBits(y0), Double.doubleToLongBits(x1),
				Double.doubleToLongBits(y1));
	}

	@Override
	public String toString() {
		return "BoundingBox{" + "x0=" + x0 + ", y0=" + y0 + ", x1=" + x1 + ", y1=" + y1 + '}';
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Metadata for an individual page, slide, or sheet.
 *
 * <p>
 * Captures per-page information including dimensions, content counts, and
 * visibility state (for presentations).
 *
 * @since 4.0.0
 */
public final class PageInfo {
	private final long number;
	private final String title;
	private final double[] dimensions;
	private final Integer imageCount;
	private final Integer tableCount;
	private final Boolean hidden;
	private final Boolean isBlank;

	@JsonCreator
	public PageInfo(@JsonProperty("number") long number, @JsonProperty("title") String title,
			@JsonProperty("dimensions") double[] dimensions, @JsonProperty("image_count") Integer imageCount,
			@JsonProperty("table_count") Integer tableCount, @JsonProperty("hidden") Boolean hidden,
			@JsonProperty("is_blank") Boolean isBlank) {
		if (number < 1) {
			throw new IllegalArgumentException("page number must be positive");
		}
		if (dimensions != null && dimensions.length != 2) {
			throw new IllegalArgumentException("dimensions must have exactly 2 elements (width, height)");
		}
		this.number = number;
		this.title = title;
		this.dimensions = dimensions;
		this.imageCount = imageCount;
		this.tableCount = tableCount;
		this.hidden = hidden;
		this.isBlank = isBlank;
	}

	/**
	 * Get the page number (1-indexed).
	 *
	 * @return page number
	 */
	public long getNumber() {
		return number;
	}

	/**
	 * Get the page title (usually for presentations).
	 *
	 * @return page title, or empty if not available
	 */
	public Optional<String> getTitle() {
		return Optional.ofNullable(title);
	}

	/**
	 * Get the dimensions in points (PDF) or pixels (images) as [width, height].
	 *
	 * @return dimensions array [width, height], or empty if not available
	 */
	public Optional<double[]> getDimensions() {
		return Optional.ofNullable(dimensions);
	}

	/**
	 * Get the page width in points (PDF) or pixels (images).
	 *
	 * @return page width, or empty if not available
	 */
	public Optional<Double> getWidth() {
		return dimensions != null && dimensions.length >= 1 ? Optional.of(dimensions[0]) : Optional.empty();
	}

	/**
	 * Get the page height in points (PDF) or pixels (images).
	 *
	 * @return page height, or empty if not available
	 */
	public Optional<Double> getHeight() {
		return dimensions != null && dimensions.length >= 2 ? Optional.of(dimensions[1]) : Optional.empty();
	}

	/**
	 * Get the number of images on this page.
	 *
	 * @return image count, or empty if not available
	 */
	public Optional<Integer> getImageCount() {
		return Optional.ofNullable(imageCount);
	}

	/**
	 * Get the number of tables on this page.
	 *
	 * @return table count, or empty if not available
	 */
	public Optional<Integer> getTableCount() {
		return Optional.ofNullable(tableCount);
	}

	/**
	 * Get whether this page is hidden (e.g., in presentations).
	 *
	 * @return true if hidden, false if visible, empty if not applicable
	 */
	public Optional<Boolean> isHidden() {
		return Optional.ofNullable(hidden);
	}

	/**
	 * Get the visibility state of this page (for presentations).
	 *
	 * Convenience method that returns the inverse of isHidden().
	 *
	 * @return true if visible, false if hidden, empty if not applicable
	 */
	public Optional<Boolean> isVisible() {
		return hidden != null ? Optional.of(!hidden) : Optional.empty();
	}

	/**
	 * Get whether this page is blank (contains no meaningful content).
	 *
	 * @return true if blank, false otherwise, empty if not applicable
	 */
	public Optional<Boolean> isBlank() {
		return Optional.ofNullable(isBlank);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof PageInfo)) {
			return false;
		}
		PageInfo other = (PageInfo) obj;
		return number == other.number && Objects.equals(title, other.title)
				&& java.util.Arrays.equals(dimensions, other.dimensions) && Objects.equals(imageCount, other.imageCount)
				&& Objects.equals(tableCount, other.tableCount) && Objects.equals(hidden, other.hidden)
				&& Objects.equals(isBlank, other.isBlank);
	}

	@Override
	public int hashCode() {
		return Objects.hash(number, title, java.util.Arrays.hashCode(dimensions), imageCount, tableCount, hidden,
				isBlank);
	}

	@Override
	public String toString() {
		return "PageInfo{" + "number=" + number + ", title=" + title + ", dimensions="
				+ java.util.Arrays.toString(dimensions) + ", imageCount=" + imageCount + ", tableCount=" + tableCount
				+ ", hidden=" + hidden + ", isBlank=" + isBlank + '}';
	}
}

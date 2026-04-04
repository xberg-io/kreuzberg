package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Individual grid cell with position and span metadata.
 *
 * <p>
 * Represents a cell in a table grid with content, position (row/column), and
 * span information.
 *
 * @since 4.3.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class GridCell {
	private final String content;
	private final int row;
	private final int col;
	private final int rowSpan;
	private final int colSpan;
	private final boolean isHeader;
	private final BoundingBox bbox;

	/**
	 * Create a new GridCell.
	 *
	 * @param content
	 *            cell text content (must not be null)
	 * @param row
	 *            zero-indexed row position
	 * @param col
	 *            zero-indexed column position
	 * @param rowSpan
	 *            number of rows this cell spans (default 1)
	 * @param colSpan
	 *            number of columns this cell spans (default 1)
	 * @param isHeader
	 *            whether this is a header cell (default false)
	 * @param bbox
	 *            bounding box for this cell, or null
	 */
	@JsonCreator
	public GridCell(@JsonProperty("content") String content, @JsonProperty("row") int row, @JsonProperty("col") int col,
			@JsonProperty("row_span") int rowSpan, @JsonProperty("col_span") int colSpan,
			@JsonProperty("is_header") boolean isHeader, @JsonProperty("bbox") BoundingBox bbox) {
		this.content = Objects.requireNonNull(content, "content must not be null");
		if (row < 0) {
			throw new IllegalArgumentException("row must be non-negative, got " + row);
		}
		if (col < 0) {
			throw new IllegalArgumentException("col must be non-negative, got " + col);
		}
		if (rowSpan < 1) {
			throw new IllegalArgumentException("rowSpan must be >= 1, got " + rowSpan);
		}
		if (colSpan < 1) {
			throw new IllegalArgumentException("colSpan must be >= 1, got " + colSpan);
		}
		this.row = row;
		this.col = col;
		this.rowSpan = rowSpan;
		this.colSpan = colSpan;
		this.isHeader = isHeader;
		this.bbox = bbox;
	}

	/**
	 * Get the cell text content.
	 *
	 * @return cell content (never null)
	 */
	@JsonProperty("content")
	public String getContent() {
		return content;
	}

	/**
	 * Get the zero-indexed row position.
	 *
	 * @return row index
	 */
	@JsonProperty("row")
	public int getRow() {
		return row;
	}

	/**
	 * Get the zero-indexed column position.
	 *
	 * @return column index
	 */
	@JsonProperty("col")
	public int getCol() {
		return col;
	}

	/**
	 * Get the number of rows this cell spans.
	 *
	 * @return row span (>= 1)
	 */
	@JsonProperty("row_span")
	public int getRowSpan() {
		return rowSpan;
	}

	/**
	 * Get the number of columns this cell spans.
	 *
	 * @return column span (>= 1)
	 */
	@JsonProperty("col_span")
	public int getColSpan() {
		return colSpan;
	}

	/**
	 * Check if this is a header cell.
	 *
	 * @return true if this is a header cell, false otherwise
	 */
	@JsonProperty("is_header")
	public boolean isHeader() {
		return isHeader;
	}

	/**
	 * Get the bounding box for this cell.
	 *
	 * @return bounding box, or empty if not available
	 */
	@JsonProperty("bbox")
	public Optional<BoundingBox> getBBox() {
		return Optional.ofNullable(bbox);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof GridCell)) {
			return false;
		}
		GridCell other = (GridCell) obj;
		return Objects.equals(content, other.content) && row == other.row && col == other.col
				&& rowSpan == other.rowSpan && colSpan == other.colSpan && isHeader == other.isHeader
				&& Objects.equals(bbox, other.bbox);
	}

	@Override
	public int hashCode() {
		return Objects.hash(content, row, col, rowSpan, colSpan, isHeader, bbox);
	}

	@Override
	public String toString() {
		return "GridCell{" + "row=" + row + ", col=" + col + ", rowSpan=" + rowSpan + ", colSpan=" + colSpan
				+ ", isHeader=" + isHeader + '}';
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Structured table grid with cell-level metadata.
 *
 * <p>
 * Stores row/column dimensions and all cells with position and span
 * information.
 *
 * @since 4.3.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class TableGrid {
	private final int rows;
	private final int cols;
	private final List<GridCell> cells;

	/**
	 * Create a new TableGrid.
	 *
	 * @param rows
	 *            number of rows in the table
	 * @param cols
	 *            number of columns in the table
	 * @param cells
	 *            all cells in the table (may be empty or null)
	 */
	@JsonCreator
	public TableGrid(@JsonProperty("rows") int rows, @JsonProperty("cols") int cols,
			@JsonProperty("cells") List<GridCell> cells) {
		if (rows < 0) {
			throw new IllegalArgumentException("rows must be non-negative, got " + rows);
		}
		if (cols < 0) {
			throw new IllegalArgumentException("cols must be non-negative, got " + cols);
		}
		this.rows = rows;
		this.cols = cols;
		this.cells = Collections.unmodifiableList(cells != null ? cells : Collections.emptyList());
	}

	/**
	 * Get the number of rows in the table.
	 *
	 * @return row count
	 */
	@JsonProperty("rows")
	public int getRows() {
		return rows;
	}

	/**
	 * Get the number of columns in the table.
	 *
	 * @return column count
	 */
	@JsonProperty("cols")
	public int getCols() {
		return cols;
	}

	/**
	 * Get all cells in the table.
	 *
	 * <p>
	 * Cells are in row-major order.
	 *
	 * @return unmodifiable list of cells (never null, but may be empty)
	 */
	@JsonProperty("cells")
	public List<GridCell> getCells() {
		return cells;
	}

	/**
	 * Get the cell count.
	 *
	 * @return total number of cells
	 */
	public int getCellCount() {
		return cells.size();
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof TableGrid)) {
			return false;
		}
		TableGrid other = (TableGrid) obj;
		return rows == other.rows && cols == other.cols && Objects.equals(cells, other.cells);
	}

	@Override
	public int hashCode() {
		return Objects.hash(rows, cols, cells);
	}

	@Override
	public String toString() {
		return "TableGrid{" + "rows=" + rows + ", cols=" + cols + ", cells=" + cells.size() + '}';
	}
}

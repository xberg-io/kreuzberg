package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Represents a table extracted from a document.
 *
 * <p>
 * Tables are represented as a 2D grid of cells with an optional Markdown
 * representation and page information.
 *
 * @param cells
 *            the table cells as a 2D list (rows × columns)
 * @param markdown
 *            the Markdown representation of the table
 * @param pageNumber
 *            the page number where the table was found (1-indexed)
 * @param boundingBox
 *            the bounding box coordinates of the table on the page, if
 *            available
 */
public record Table(@JsonProperty("cells") List<List<String>> cells, @JsonProperty("markdown") String markdown,
		@JsonProperty("page_number") int pageNumber, @JsonProperty("bounding_box") BoundingBox boundingBox) {
	/**
	 * Creates a new Table.
	 *
	 * @param cells
	 *            the table cells (must not be null)
	 * @param markdown
	 *            the Markdown representation (must not be null)
	 * @param pageNumber
	 *            the page number (0 for non-paginated documents, >= 1 for paginated
	 *            documents)
	 * @param boundingBox
	 *            the bounding box coordinates, or null if not available
	 * @throws NullPointerException
	 *             if cells or markdown is null
	 * @throws IllegalArgumentException
	 *             if pageNumber is negative
	 */
	@JsonCreator
	public Table(@JsonProperty("cells") List<List<String>> cells, @JsonProperty("markdown") String markdown,
			@JsonProperty("page_number") int pageNumber, @JsonProperty("bounding_box") BoundingBox boundingBox) {
		Objects.requireNonNull(cells, "cells must not be null");
		Objects.requireNonNull(markdown, "markdown must not be null");
		if (pageNumber < 0) {
			throw new IllegalArgumentException("pageNumber must be non-negative, got " + pageNumber);
		}
		this.cells = deepCopyTable(cells);
		this.markdown = markdown;
		this.pageNumber = pageNumber;
		this.boundingBox = boundingBox;
	}

	/**
	 * Creates a Table from raw values.
	 *
	 * @param cells
	 *            the table cells
	 * @param markdown
	 *            the Markdown representation
	 * @param pageNumber
	 *            the page number
	 * @param boundingBox
	 *            the bounding box coordinates, or null if not available
	 * @return a new Table
	 */
	public static Table of(List<List<String>> cells, String markdown, int pageNumber, BoundingBox boundingBox) {
		return new Table(cells, markdown, pageNumber, boundingBox);
	}

	/**
	 * Returns the number of rows in the table.
	 *
	 * @return the row count
	 */
	public int getRowCount() {
		return cells.size();
	}

	/**
	 * Returns the number of columns in the table.
	 *
	 * <p>
	 * Returns 0 if the table is empty, otherwise returns the column count of the
	 * first row (assumes all rows have the same column count).
	 *
	 * @return the column count
	 */
	public int getColumnCount() {
		return cells.isEmpty() ? 0 : cells.get(0).size();
	}

	/**
	 * Returns the cell at the specified row and column.
	 *
	 * @param row
	 *            the row index (0-based)
	 * @param col
	 *            the column index (0-based)
	 * @return the cell value
	 * @throws IndexOutOfBoundsException
	 *             if row or column is out of bounds
	 */
	public String getCell(int row, int col) {
		return cells.get(row).get(col);
	}

	/**
	 * Returns a row from the table.
	 *
	 * @param row
	 *            the row index (0-based)
	 * @return an unmodifiable list of cell values
	 * @throws IndexOutOfBoundsException
	 *             if row is out of bounds
	 */
	public List<String> getRow(int row) {
		return Collections.unmodifiableList(cells.get(row));
	}

	/**
	 * Returns the bounding box of the table.
	 *
	 * @return the bounding box, or null if not available
	 */
	public BoundingBox getBoundingBox() {
		return boundingBox;
	}

	@Override
	public String toString() {
		return "Table{" + "rows=" + getRowCount() + ", cols=" + getColumnCount() + ", page=" + pageNumber
				+ ", boundingBox=" + boundingBox + '}';
	}

	private static List<List<String>> deepCopyTable(List<List<String>> table) {
		List<List<String>> copy = new ArrayList<>(table.size());
		for (List<String> row : table) {
			copy.add(Collections.unmodifiableList(new ArrayList<>(row)));
		}
		return Collections.unmodifiableList(copy);
	}
}

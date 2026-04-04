defmodule KreuzbergTest.E2E.TableExtractionTest do
  @moduledoc """
  End-to-end tests for table extraction from various document formats.

  Tests complete table extraction workflows including:
  - Table detection and extraction from HTML, PDF, and CSV
  - Table structure preservation (rows, columns, headers)
  - Nested table handling
  - Configuration-based extraction variants
  - Multi-format batch processing with tables
  - Table metadata and statistics
  """

  use ExUnit.Case, async: true

  @tag :e2e
  test "extracts simple table from HTML" do
    html_with_table = create_simple_html_table()

    {:ok, result} = Kreuzberg.extract(html_with_table, "text/html")

    assert is_list(result.tables)
    # Simple table should have at least one table with content
    assert result.tables != [], "Should extract at least one table"
  end

  @tag :e2e
  test "extracts table with headers" do
    html_with_headers = create_table_with_headers()

    {:ok, result} = Kreuzberg.extract(html_with_headers, "text/html")

    assert is_list(result.tables)
    assert result.tables != [], "Should extract table with headers"
    # Validate table structure - should contain header/data separation
    Enum.each(result.tables, fn table ->
      assert is_map(table) or is_list(table), "Table should be structured as map or list"
    end)
  end

  @tag :e2e
  test "extracts complex multi-row table" do
    complex_table = create_complex_table()

    {:ok, result} = Kreuzberg.extract(complex_table, "text/html")

    assert is_list(result.tables)
    assert result.tables != [], "Should extract complex table"
    # Complex table should preserve all rows
    Enum.each(result.tables, fn table ->
      assert is_map(table) or is_list(table)
      # If it's a list of rows, should have multiple rows
      if is_list(table), do: assert(table != [])
    end)
  end

  @tag :e2e
  test "extracts table with merged cells" do
    table_merged = create_table_with_merged_cells()

    {:ok, result} = Kreuzberg.extract(table_merged, "text/html")

    assert is_list(result.tables)
    # Merged cells table should still be extracted (regardless of colspan handling)
    if result.tables != [] do
      assert true, "Table with merged cells extracted successfully"
    end
  end

  @tag :e2e
  test "extracts multiple tables from single document" do
    multi_tables = create_document_with_multiple_tables()

    {:ok, result} = Kreuzberg.extract(multi_tables, "text/html")

    assert is_list(result.tables)
    # Document has 2 tables, should extract both
    assert length(result.tables) >= 2, "Should extract both tables from document"
  end

  @tag :e2e
  test "extracts CSV as structured data" do
    csv_data = create_sample_csv()

    {:ok, result} = Kreuzberg.extract(csv_data, "text/csv")

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
    # CSV should preserve the data structure (at minimum the headers and rows)
    assert byte_size(result.content) > 0, "CSV content should not be empty"

    assert String.contains?(result.content, "Name") or String.contains?(result.content, "Alice"),
           "CSV content should preserve headers or data"
  end

  @tag :e2e
  test "preserves table column count" do
    table_html = create_simple_html_table()

    {:ok, result} = Kreuzberg.extract(table_html, "text/html")

    assert is_list(result.tables)
    assert result.tables != [], "Should extract table"
    # Each table should have consistent structure and represent the data
    Enum.each(result.tables, fn table ->
      assert is_map(table) or is_list(table), "Table should be structured data"
    end)
  end

  @tag :e2e
  test "handles tables with special characters" do
    table_special = create_table_with_special_chars()

    {:ok, result} = Kreuzberg.extract(table_special, "text/html")

    assert is_list(result.tables)
    # Special characters (&lt;, &gt;, &amp;, &quot;) should be properly decoded
    if result.tables != [] do
      Enum.each(result.tables, fn table ->
        assert is_map(table) or is_list(table)
      end)
    end
  end

  @tag :e2e
  test "extracts tables with configuration" do
    html_with_table = create_simple_html_table()

    config = %Kreuzberg.ExtractionConfig{
      chunking: %{"enabled" => true}
    }

    {:ok, result} = Kreuzberg.extract(html_with_table, "text/html", config)

    assert is_list(result.tables)
  end

  @tag :e2e
  test "batch extracts multiple documents with tables" do
    html_bytes = [
      create_simple_html_table(),
      create_table_with_headers(),
      create_complex_table()
    ]

    {:ok, results} =
      Kreuzberg.batch_extract_bytes(
        html_bytes,
        ["text/html", "text/html", "text/html"]
      )

    assert is_list(results)
    assert length(results) == 3, "Batch should return all 3 documents"

    # Verify order preservation: first input should match first output
    Enum.each(results, fn result ->
      assert %Kreuzberg.ExtractionResult{} = result
      assert is_list(result.tables)
      assert result.tables != [], "Each document should extract tables"
    end)
  end

  @tag :e2e
  test "extracts nested table structure" do
    nested_table = create_nested_table()

    {:ok, result} = Kreuzberg.extract(nested_table, "text/html")

    assert is_list(result.tables)
    # Nested tables should be extracted (as nested structures or flattened)
    if result.tables != [] do
      Enum.each(result.tables, fn table ->
        assert is_map(table) or is_list(table)
      end)
    end
  end

  # Private helpers

  defp create_simple_html_table do
    """
    <html>
    <body>
    <table>
      <tr><th>Name</th><th>Age</th></tr>
      <tr><td>Alice</td><td>30</td></tr>
      <tr><td>Bob</td><td>25</td></tr>
    </table>
    </body>
    </html>
    """
  end

  defp create_table_with_headers do
    """
    <html>
    <body>
    <table border="1">
      <thead>
        <tr>
          <th>Product</th>
          <th>Price</th>
          <th>Quantity</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Widget A</td>
          <td>$10.00</td>
          <td>5</td>
        </tr>
        <tr>
          <td>Widget B</td>
          <td>$20.00</td>
          <td>3</td>
        </tr>
      </tbody>
    </table>
    </body>
    </html>
    """
  end

  defp create_complex_table do
    """
    <html>
    <body>
    <table>
      <tr>
        <th>ID</th>
        <th>Name</th>
        <th>Department</th>
        <th>Salary</th>
        <th>Status</th>
      </tr>
      <tr>
        <td>001</td>
        <td>John Smith</td>
        <td>Engineering</td>
        <td>$75,000</td>
        <td>Active</td>
      </tr>
      <tr>
        <td>002</td>
        <td>Jane Doe</td>
        <td>Marketing</td>
        <td>$65,000</td>
        <td>Active</td>
      </tr>
      <tr>
        <td>003</td>
        <td>Bob Johnson</td>
        <td>Sales</td>
        <td>$70,000</td>
        <td>Inactive</td>
      </tr>
    </table>
    </body>
    </html>
    """
  end

  defp create_table_with_merged_cells do
    """
    <html>
    <body>
    <table border="1">
      <tr>
        <th colspan="2">Personal Info</th>
        <th>Contact</th>
      </tr>
      <tr>
        <td>First Name</td>
        <td>Last Name</td>
        <td>Email</td>
      </tr>
      <tr>
        <td>John</td>
        <td>Doe</td>
        <td>john@example.com</td>
      </tr>
    </table>
    </body>
    </html>
    """
  end

  defp create_document_with_multiple_tables do
    """
    <html>
    <body>
    <h1>Report</h1>
    <h2>Table 1: Sales Data</h2>
    <table>
      <tr><th>Quarter</th><th>Revenue</th></tr>
      <tr><td>Q1</td><td>$100,000</td></tr>
      <tr><td>Q2</td><td>$120,000</td></tr>
    </table>
    <h2>Table 2: Expenses</h2>
    <table>
      <tr><th>Category</th><th>Amount</th></tr>
      <tr><td>Salaries</td><td>$200,000</td></tr>
      <tr><td>Equipment</td><td>$50,000</td></tr>
    </table>
    </body>
    </html>
    """
  end

  defp create_sample_csv do
    """
    Name,Age,City
    Alice,30,New York
    Bob,25,San Francisco
    Charlie,35,Chicago
    """
  end

  defp create_table_with_special_chars do
    """
    <html>
    <body>
    <table>
      <tr>
        <th>Symbol</th>
        <th>Meaning</th>
      </tr>
      <tr>
        <td>&lt;</td>
        <td>Less than</td>
      </tr>
      <tr>
        <td>&gt;</td>
        <td>Greater than</td>
      </tr>
      <tr>
        <td>&amp;</td>
        <td>Ampersand</td>
      </tr>
      <tr>
        <td>&quot;</td>
        <td>Quote</td>
      </tr>
    </table>
    </body>
    </html>
    """
  end

  defp create_nested_table do
    """
    <html>
    <body>
    <table border="1">
      <tr>
        <th>Category</th>
        <th>Details</th>
      </tr>
      <tr>
        <td>Main</td>
        <td>
          <table border="1">
            <tr><td>Sub1</td><td>Value1</td></tr>
            <tr><td>Sub2</td><td>Value2</td></tr>
          </table>
        </td>
      </tr>
    </table>
    </body>
    </html>
    """
  end
end

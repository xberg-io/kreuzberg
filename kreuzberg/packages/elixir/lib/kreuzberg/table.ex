defmodule Kreuzberg.Table do
  @moduledoc """
  Structure representing an extracted table from a document.

  Matches the Rust `Table` struct with cells, markdown, page number, and optional bounding box.

  ## Fields

    * `:cells` - Two-dimensional list of table cells [[cell1, cell2], ...]
    * `:markdown` - Markdown representation of the table
    * `:page_number` - Page number where table appears (0-indexed)
    * `:bounding_box` - Bounding box coordinates {x0, y0, x1, y1} if available, nil otherwise

  ## Examples

      iex> table = %Kreuzberg.Table{
      ...>   cells: [["Name", "Age"], ["Alice", "30"]],
      ...>   markdown: "| Name | Age |\\n|------|-----|\\n| Alice | 30 |",
      ...>   page_number: 0
      ...> }
      iex> table.cells
      [["Name", "Age"], ["Alice", "30"]]
  """

  @type t :: %__MODULE__{
          cells: list(list(String.t())),
          markdown: String.t(),
          page_number: non_neg_integer(),
          bounding_box: map() | nil
        }

  defstruct cells: [], markdown: "", page_number: 0, bounding_box: nil

  @doc """
  Creates a Table struct from a map.

  ## Examples

      iex> Kreuzberg.Table.from_map(%{"cells" => [["A", "B"]], "markdown" => "| A | B |", "page_number" => 0})
      %Kreuzberg.Table{cells: [["A", "B"]], markdown: "| A | B |", page_number: 0}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      cells: data["cells"] || [],
      markdown: data["markdown"] || "",
      page_number: data["page_number"] || 0,
      bounding_box: data["bounding_box"]
    }
  end

  @doc """
  Converts a Table struct to a map.
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = table) do
    %{
      "cells" => table.cells,
      "markdown" => table.markdown,
      "page_number" => table.page_number,
      "bounding_box" => table.bounding_box
    }
  end

  @doc """
  Returns the number of rows in the table.

  ## Examples

      iex> table = %Kreuzberg.Table{cells: [["A", "B"], ["1", "2"]]}
      iex> Kreuzberg.Table.row_count(table)
      2
  """
  @spec row_count(t()) :: non_neg_integer()
  def row_count(%__MODULE__{cells: cells}) when is_list(cells), do: length(cells)

  @doc """
  Returns the number of columns in the table.

  ## Examples

      iex> table = %Kreuzberg.Table{cells: [["A", "B"], ["1", "2"]]}
      iex> Kreuzberg.Table.column_count(table)
      2
  """
  @spec column_count(t()) :: non_neg_integer()
  def column_count(%__MODULE__{cells: []}), do: 0

  def column_count(%__MODULE__{cells: [first | _]}) when is_list(first), do: length(first)

  def column_count(%__MODULE__{cells: _}), do: 0
end

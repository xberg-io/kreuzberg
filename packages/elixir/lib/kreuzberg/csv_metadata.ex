defmodule Kreuzberg.CsvMetadata do
  @moduledoc """
  CSV/TSV file metadata.

  Matches the Rust `CsvMetadata` struct.

  ## Fields

    * `:row_count` - Number of rows
    * `:column_count` - Number of columns
    * `:delimiter` - Delimiter character (e.g., "," or "\\t")
    * `:has_header` - Whether the file has a header row
    * `:column_types` - Detected column types
  """

  @type t :: %__MODULE__{
          row_count: non_neg_integer(),
          column_count: non_neg_integer(),
          delimiter: String.t() | nil,
          has_header: boolean(),
          column_types: list(String.t()) | nil
        }

  defstruct row_count: 0,
            column_count: 0,
            delimiter: nil,
            has_header: false,
            column_types: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      row_count: data["row_count"] || 0,
      column_count: data["column_count"] || 0,
      delimiter: data["delimiter"],
      has_header: Map.get(data, "has_header", false),
      column_types: data["column_types"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "row_count" => meta.row_count,
      "column_count" => meta.column_count,
      "delimiter" => meta.delimiter,
      "has_header" => meta.has_header,
      "column_types" => meta.column_types
    }
  end
end

defmodule Kreuzberg.CodeSpan do
  @moduledoc """
  Byte and line/column span for a code element.

  ## Fields

    * `:start_byte` - Starting byte offset
    * `:end_byte` - Ending byte offset
    * `:start_line` - Starting line number
    * `:start_column` - Starting column number
    * `:end_line` - Ending line number
    * `:end_column` - Ending column number
  """

  @type t :: %__MODULE__{
          start_byte: non_neg_integer(),
          end_byte: non_neg_integer(),
          start_line: non_neg_integer(),
          start_column: non_neg_integer(),
          end_line: non_neg_integer(),
          end_column: non_neg_integer()
        }

  defstruct start_byte: 0,
            end_byte: 0,
            start_line: 0,
            start_column: 0,
            end_line: 0,
            end_column: 0

  @doc """
  Creates a CodeSpan struct from a map.

  ## Examples

      iex> Kreuzberg.CodeSpan.from_map(%{"start_byte" => 0, "end_byte" => 42, "start_line" => 1, "start_column" => 0, "end_line" => 3, "end_column" => 5})
      %Kreuzberg.CodeSpan{start_byte: 0, end_byte: 42, start_line: 1, start_column: 0, end_line: 3, end_column: 5}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      start_byte: Map.get(data, "start_byte", Map.get(data, :start_byte, 0)),
      end_byte: Map.get(data, "end_byte", Map.get(data, :end_byte, 0)),
      start_line: Map.get(data, "start_line", Map.get(data, :start_line, 0)),
      start_column: Map.get(data, "start_column", Map.get(data, :start_column, 0)),
      end_line: Map.get(data, "end_line", Map.get(data, :end_line, 0)),
      end_column: Map.get(data, "end_column", Map.get(data, :end_column, 0))
    }
  end

  @doc """
  Converts a CodeSpan struct to a map.
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = span) do
    %{
      "start_byte" => span.start_byte,
      "end_byte" => span.end_byte,
      "start_line" => span.start_line,
      "start_column" => span.start_column,
      "end_line" => span.end_line,
      "end_column" => span.end_column
    }
  end
end

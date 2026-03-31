defmodule Kreuzberg.CodeFileMetrics do
  @moduledoc """
  File-level code metrics from tree-sitter analysis.

  ## Fields

    * `:total_lines` - Total number of lines
    * `:code_lines` - Number of code lines
    * `:comment_lines` - Number of comment lines
    * `:blank_lines` - Number of blank lines
    * `:total_bytes` - Total byte size
    * `:node_count` - Number of AST nodes
    * `:error_count` - Number of parse errors
    * `:max_depth` - Maximum AST depth
  """

  @type t :: %__MODULE__{
          total_lines: non_neg_integer(),
          code_lines: non_neg_integer(),
          comment_lines: non_neg_integer(),
          blank_lines: non_neg_integer(),
          total_bytes: non_neg_integer(),
          node_count: non_neg_integer(),
          error_count: non_neg_integer(),
          max_depth: non_neg_integer()
        }

  defstruct total_lines: 0,
            code_lines: 0,
            comment_lines: 0,
            blank_lines: 0,
            total_bytes: 0,
            node_count: 0,
            error_count: 0,
            max_depth: 0

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      total_lines: Map.get(data, "total_lines", Map.get(data, :total_lines, 0)),
      code_lines: Map.get(data, "code_lines", Map.get(data, :code_lines, 0)),
      comment_lines: Map.get(data, "comment_lines", Map.get(data, :comment_lines, 0)),
      blank_lines: Map.get(data, "blank_lines", Map.get(data, :blank_lines, 0)),
      total_bytes: Map.get(data, "total_bytes", Map.get(data, :total_bytes, 0)),
      node_count: Map.get(data, "node_count", Map.get(data, :node_count, 0)),
      error_count: Map.get(data, "error_count", Map.get(data, :error_count, 0)),
      max_depth: Map.get(data, "max_depth", Map.get(data, :max_depth, 0))
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = metrics) do
    %{
      "total_lines" => metrics.total_lines,
      "code_lines" => metrics.code_lines,
      "comment_lines" => metrics.comment_lines,
      "blank_lines" => metrics.blank_lines,
      "total_bytes" => metrics.total_bytes,
      "node_count" => metrics.node_count,
      "error_count" => metrics.error_count,
      "max_depth" => metrics.max_depth
    }
  end
end

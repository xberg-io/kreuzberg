defmodule Kreuzberg.CodeChunkContext do
  @moduledoc """
  Context for a code chunk (parent scope information).

  ## Fields

    * `:parent_name` - Name of the parent scope
    * `:parent_kind` - Kind of the parent scope
  """

  @type t :: %__MODULE__{
          parent_name: String.t() | nil,
          parent_kind: String.t() | nil
        }

  defstruct parent_name: nil,
            parent_kind: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      parent_name: Map.get(data, "parent_name", Map.get(data, :parent_name)),
      parent_kind: Map.get(data, "parent_kind", Map.get(data, :parent_kind))
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = ctx) do
    %{
      "parent_name" => ctx.parent_name,
      "parent_kind" => ctx.parent_kind
    }
  end
end

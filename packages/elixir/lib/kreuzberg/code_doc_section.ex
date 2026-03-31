defmodule Kreuzberg.CodeDocSection do
  @moduledoc """
  Section within a docstring.

  ## Fields

    * `:kind` - Section kind (e.g. "param", "returns", "description")
    * `:name` - Section name (e.g. parameter name)
    * `:content` - Section content text
  """

  @type t :: %__MODULE__{
          kind: String.t(),
          name: String.t() | nil,
          content: String.t()
        }

  defstruct kind: "",
            name: nil,
            content: ""

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      kind: Map.get(data, "kind", Map.get(data, :kind, "")),
      name: Map.get(data, "name", Map.get(data, :name)),
      content: Map.get(data, "content", Map.get(data, :content, ""))
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = section) do
    %{
      "kind" => section.kind,
      "name" => section.name,
      "content" => section.content
    }
  end
end

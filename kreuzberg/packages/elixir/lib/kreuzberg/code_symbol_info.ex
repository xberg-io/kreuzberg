defmodule Kreuzberg.CodeSymbolInfo do
  @moduledoc """
  Symbol definition information.

  ## Fields

    * `:name` - Symbol name
    * `:kind` - Symbol kind (e.g. "variable", "constant")
    * `:type_annotation` - Type annotation if present
    * `:span` - Source span
  """

  @type t :: %__MODULE__{
          name: String.t(),
          kind: String.t(),
          type_annotation: String.t() | nil,
          span: Kreuzberg.CodeSpan.t()
        }

  defstruct name: "",
            kind: "",
            type_annotation: nil,
            span: %Kreuzberg.CodeSpan{}

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    span =
      case Map.get(data, "span", Map.get(data, :span)) do
        %Kreuzberg.CodeSpan{} = s -> s
        map when is_map(map) -> Kreuzberg.CodeSpan.from_map(map)
        _ -> %Kreuzberg.CodeSpan{}
      end

    %__MODULE__{
      name: Map.get(data, "name", Map.get(data, :name, "")),
      kind: Map.get(data, "kind", Map.get(data, :kind, "")),
      type_annotation: Map.get(data, "type_annotation", Map.get(data, :type_annotation)),
      span: span
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "name" => info.name,
      "kind" => info.kind,
      "type_annotation" => info.type_annotation,
      "span" => Kreuzberg.CodeSpan.to_map(info.span)
    }
  end
end

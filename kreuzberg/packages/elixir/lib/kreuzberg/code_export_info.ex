defmodule Kreuzberg.CodeExportInfo do
  @moduledoc """
  Export statement information.

  ## Fields

    * `:name` - Exported item name
    * `:kind` - Export kind
    * `:span` - Source span
  """

  @type t :: %__MODULE__{
          name: String.t(),
          kind: String.t(),
          span: Kreuzberg.CodeSpan.t()
        }

  defstruct name: "",
            kind: "",
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
      span: span
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "name" => info.name,
      "kind" => info.kind,
      "span" => Kreuzberg.CodeSpan.to_map(info.span)
    }
  end
end

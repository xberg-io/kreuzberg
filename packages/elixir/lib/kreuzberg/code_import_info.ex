defmodule Kreuzberg.CodeImportInfo do
  @moduledoc """
  Import statement information.

  ## Fields

    * `:source` - Import source/module path
    * `:items` - Imported item names
    * `:alias` - Import alias
    * `:is_wildcard` - Whether this is a wildcard import
    * `:span` - Source span
  """

  @type t :: %__MODULE__{
          source: String.t(),
          items: [String.t()],
          alias: String.t() | nil,
          is_wildcard: boolean(),
          span: Kreuzberg.CodeSpan.t()
        }

  defstruct source: "",
            items: [],
            alias: nil,
            is_wildcard: false,
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
      source: Map.get(data, "source", Map.get(data, :source, "")),
      items: Map.get(data, "items", Map.get(data, :items, [])),
      alias: Map.get(data, "alias", Map.get(data, :alias)),
      is_wildcard: Map.get(data, "is_wildcard", Map.get(data, :is_wildcard, false)),
      span: span
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "source" => info.source,
      "items" => info.items,
      "alias" => info.alias,
      "is_wildcard" => info.is_wildcard,
      "span" => Kreuzberg.CodeSpan.to_map(info.span)
    }
  end
end

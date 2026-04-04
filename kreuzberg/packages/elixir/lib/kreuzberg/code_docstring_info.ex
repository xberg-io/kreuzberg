defmodule Kreuzberg.CodeDocstringInfo do
  @moduledoc """
  Docstring information with parsed sections.

  ## Fields

    * `:text` - Raw docstring text
    * `:format` - Docstring format (e.g. "javadoc", "numpy", "google")
    * `:associated_item` - Name of the associated code item
    * `:span` - Source span
    * `:sections` - Parsed docstring sections
  """

  @type t :: %__MODULE__{
          text: String.t(),
          format: String.t(),
          associated_item: String.t() | nil,
          span: Kreuzberg.CodeSpan.t(),
          sections: [Kreuzberg.CodeDocSection.t()]
        }

  defstruct text: "",
            format: "",
            associated_item: nil,
            span: %Kreuzberg.CodeSpan{},
            sections: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    span =
      case Map.get(data, "span", Map.get(data, :span)) do
        %Kreuzberg.CodeSpan{} = s -> s
        map when is_map(map) -> Kreuzberg.CodeSpan.from_map(map)
        _ -> %Kreuzberg.CodeSpan{}
      end

    sections =
      case Map.get(data, "sections", Map.get(data, :sections, [])) do
        list when is_list(list) -> Enum.map(list, &Kreuzberg.CodeDocSection.from_map/1)
        _ -> []
      end

    %__MODULE__{
      text: Map.get(data, "text", Map.get(data, :text, "")),
      format: Map.get(data, "format", Map.get(data, :format, "")),
      associated_item: Map.get(data, "associated_item", Map.get(data, :associated_item)),
      span: span,
      sections: sections
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "text" => info.text,
      "format" => info.format,
      "associated_item" => info.associated_item,
      "span" => Kreuzberg.CodeSpan.to_map(info.span),
      "sections" => Enum.map(info.sections, &Kreuzberg.CodeDocSection.to_map/1)
    }
  end
end

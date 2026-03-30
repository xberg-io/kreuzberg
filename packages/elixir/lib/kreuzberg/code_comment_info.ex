defmodule Kreuzberg.CodeCommentInfo do
  @moduledoc """
  Comment information.

  ## Fields

    * `:text` - Comment text
    * `:kind` - Comment kind (e.g. "line", "block")
    * `:span` - Source span
  """

  @type t :: %__MODULE__{
          text: String.t(),
          kind: String.t(),
          span: Kreuzberg.CodeSpan.t()
        }

  defstruct text: "",
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
      text: Map.get(data, "text", Map.get(data, :text, "")),
      kind: Map.get(data, "kind", Map.get(data, :kind, "")),
      span: span
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "text" => info.text,
      "kind" => info.kind,
      "span" => Kreuzberg.CodeSpan.to_map(info.span)
    }
  end
end

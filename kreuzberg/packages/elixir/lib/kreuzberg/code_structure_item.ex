defmodule Kreuzberg.CodeStructureItem do
  @moduledoc """
  Structural code element (function, class, method, etc.).

  ## Fields

    * `:kind` - Item kind (e.g. "function", "class", "method")
    * `:name` - Item name
    * `:visibility` - Visibility modifier (e.g. "public", "private")
    * `:span` - Source span
    * `:children` - Nested structure items
    * `:decorators` - Decorators/annotations
    * `:doc_comment` - Associated doc comment
    * `:signature` - Function/method signature
    * `:body_span` - Span of the body block
  """

  @type t :: %__MODULE__{
          kind: String.t(),
          name: String.t() | nil,
          visibility: String.t() | nil,
          span: Kreuzberg.CodeSpan.t(),
          children: [t()],
          decorators: [String.t()],
          doc_comment: String.t() | nil,
          signature: String.t() | nil,
          body_span: Kreuzberg.CodeSpan.t() | nil
        }

  defstruct kind: "",
            name: nil,
            visibility: nil,
            span: %Kreuzberg.CodeSpan{},
            children: [],
            decorators: [],
            doc_comment: nil,
            signature: nil,
            body_span: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    span =
      case Map.get(data, "span", Map.get(data, :span)) do
        %Kreuzberg.CodeSpan{} = s -> s
        map when is_map(map) -> Kreuzberg.CodeSpan.from_map(map)
        _ -> %Kreuzberg.CodeSpan{}
      end

    children =
      case Map.get(data, "children", Map.get(data, :children, [])) do
        list when is_list(list) -> Enum.map(list, &from_map/1)
        _ -> []
      end

    body_span =
      case Map.get(data, "body_span", Map.get(data, :body_span)) do
        %Kreuzberg.CodeSpan{} = s -> s
        map when is_map(map) -> Kreuzberg.CodeSpan.from_map(map)
        _ -> nil
      end

    %__MODULE__{
      kind: Map.get(data, "kind", Map.get(data, :kind, "")),
      name: Map.get(data, "name", Map.get(data, :name)),
      visibility: Map.get(data, "visibility", Map.get(data, :visibility)),
      span: span,
      children: children,
      decorators: Map.get(data, "decorators", Map.get(data, :decorators, [])),
      doc_comment: Map.get(data, "doc_comment", Map.get(data, :doc_comment)),
      signature: Map.get(data, "signature", Map.get(data, :signature)),
      body_span: body_span
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = item) do
    %{
      "kind" => item.kind,
      "name" => item.name,
      "visibility" => item.visibility,
      "span" => Kreuzberg.CodeSpan.to_map(item.span),
      "children" => Enum.map(item.children, &to_map/1),
      "decorators" => item.decorators,
      "doc_comment" => item.doc_comment,
      "signature" => item.signature,
      "body_span" =>
        if item.body_span do
          Kreuzberg.CodeSpan.to_map(item.body_span)
        end
    }
  end
end

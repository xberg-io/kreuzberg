defmodule Kreuzberg.CodeChunk do
  @moduledoc """
  Code chunk with source span and optional parent context.

  ## Fields

    * `:content` - Chunk text content
    * `:language` - Programming language
    * `:span` - Source span
    * `:context` - Optional parent context
  """

  @type t :: %__MODULE__{
          content: String.t(),
          language: String.t(),
          span: Kreuzberg.CodeSpan.t(),
          context: Kreuzberg.CodeChunkContext.t() | nil
        }

  defstruct content: "",
            language: "",
            span: %Kreuzberg.CodeSpan{},
            context: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    span =
      case Map.get(data, "span", Map.get(data, :span)) do
        %Kreuzberg.CodeSpan{} = s -> s
        map when is_map(map) -> Kreuzberg.CodeSpan.from_map(map)
        _ -> %Kreuzberg.CodeSpan{}
      end

    context =
      case Map.get(data, "context", Map.get(data, :context)) do
        %Kreuzberg.CodeChunkContext{} = c -> c
        map when is_map(map) -> Kreuzberg.CodeChunkContext.from_map(map)
        _ -> nil
      end

    %__MODULE__{
      content: Map.get(data, "content", Map.get(data, :content, "")),
      language: Map.get(data, "language", Map.get(data, :language, "")),
      span: span,
      context: context
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = chunk) do
    %{
      "content" => chunk.content,
      "language" => chunk.language,
      "span" => Kreuzberg.CodeSpan.to_map(chunk.span),
      "context" =>
        if chunk.context do
          Kreuzberg.CodeChunkContext.to_map(chunk.context)
        end
    }
  end
end

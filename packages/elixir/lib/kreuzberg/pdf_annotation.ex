defmodule Kreuzberg.PdfAnnotation do
  @moduledoc """
  Structure representing a PDF annotation extracted from a document page.

  Matches the Rust `PdfAnnotation` struct.

  ## Fields

    * `:annotation_type` - The type of annotation (e.g., "text", "highlight", "link", "stamp", "underline", "strike_out", "other")
    * `:content` - Text content of the annotation (e.g., comment text, link URL), or nil
    * `:page_number` - Page number where the annotation appears (1-indexed)
    * `:bounding_box` - Bounding box coordinates {x0, y0, x1, y1} if available, nil otherwise

  ## Examples

      iex> annotation = %Kreuzberg.PdfAnnotation{
      ...>   annotation_type: "highlight",
      ...>   content: "Important section",
      ...>   page_number: 3,
      ...>   bounding_box: %{"x0" => 72.0, "y0" => 200.0, "x1" => 540.0, "y1" => 220.0}
      ...> }
      iex> annotation.annotation_type
      "highlight"
  """

  @type t :: %__MODULE__{
          annotation_type: String.t(),
          content: String.t() | nil,
          page_number: non_neg_integer(),
          bounding_box: map() | nil
        }

  defstruct [
    :content,
    :bounding_box,
    annotation_type: "other",
    page_number: 0
  ]

  @doc """
  Creates a PdfAnnotation struct from a map.

  ## Examples

      iex> Kreuzberg.PdfAnnotation.from_map(%{
      ...>   "annotation_type" => "text",
      ...>   "content" => "A note",
      ...>   "page_number" => 1
      ...> })
      %Kreuzberg.PdfAnnotation{
        annotation_type: "text",
        content: "A note",
        page_number: 1,
        bounding_box: nil
      }
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      annotation_type: data["annotation_type"] || "other",
      content: data["content"],
      page_number: data["page_number"] || 0,
      bounding_box: data["bounding_box"]
    }
  end

  @doc """
  Converts a PdfAnnotation struct to a map.

  ## Examples

      iex> annotation = %Kreuzberg.PdfAnnotation{
      ...>   annotation_type: "link",
      ...>   content: "https://example.com",
      ...>   page_number: 2
      ...> }
      iex> Kreuzberg.PdfAnnotation.to_map(annotation)
      %{
        "annotation_type" => "link",
        "content" => "https://example.com",
        "page_number" => 2,
        "bounding_box" => nil
      }
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = annotation) do
    %{
      "annotation_type" => annotation.annotation_type,
      "content" => annotation.content,
      "page_number" => annotation.page_number,
      "bounding_box" => annotation.bounding_box
    }
  end
end

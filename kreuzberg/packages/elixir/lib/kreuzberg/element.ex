defmodule Kreuzberg.BoundingBox do
  @moduledoc """
  Bounding box coordinates for element positioning in documents.

  Represents the rectangular region where an element appears within a document,
  using float coordinates for precise positioning.

  ## Fields

    * `:x0` - Left x-coordinate (0.0 is the left edge)
    * `:y0` - Bottom y-coordinate (0.0 is the bottom edge)
    * `:x1` - Right x-coordinate (document width is the right edge)
    * `:y1` - Top y-coordinate (document height is the top edge)

  ## Examples

      iex> bbox = %Kreuzberg.BoundingBox{
      ...>   x0: 10.5,
      ...>   y0: 20.5,
      ...>   x1: 100.5,
      ...>   y1: 50.5
      ...> }
      iex> bbox.x0
      10.5
  """

  @type t :: %__MODULE__{
          x0: float(),
          y0: float(),
          x1: float(),
          y1: float()
        }

  defstruct [:x0, :y0, :x1, :y1]

  @doc """
  Creates a BoundingBox struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct.

  ## Parameters

    * `data` - A map containing x0, y0, x1, y1 fields

  ## Returns

  A `BoundingBox` struct with float coordinates.

  ## Examples

      iex> bbox_map = %{"x0" => 10.5, "y0" => 20.5, "x1" => 100.5, "y1" => 50.5}
      iex> Kreuzberg.BoundingBox.from_map(bbox_map)
      %Kreuzberg.BoundingBox{x0: 10.5, y0: 20.5, x1: 100.5, y1: 50.5}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      x0: to_float(data["x0"]),
      y0: to_float(data["y0"]),
      x1: to_float(data["x1"]),
      y1: to_float(data["y1"])
    }
  end

  @doc """
  Converts a BoundingBox struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `bbox` - A `BoundingBox` struct

  ## Returns

  A map with string keys and float values.

  ## Examples

      iex> bbox = %Kreuzberg.BoundingBox{x0: 10.5, y0: 20.5, x1: 100.5, y1: 50.5}
      iex> Kreuzberg.BoundingBox.to_map(bbox)
      %{"x0" => 10.5, "y0" => 20.5, "x1" => 100.5, "y1" => 50.5}
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = bbox) do
    %{
      "x0" => bbox.x0,
      "y0" => bbox.y0,
      "x1" => bbox.x1,
      "y1" => bbox.y1
    }
  end

  @doc """
  Calculates the width of the bounding box.

  ## Parameters

    * `bbox` - A `BoundingBox` struct

  ## Returns

  The width (x1 - x0).

  ## Examples

      iex> bbox = %Kreuzberg.BoundingBox{x0: 10.0, y0: 20.0, x1: 110.0, y1: 50.0}
      iex> Kreuzberg.BoundingBox.width(bbox)
      100.0
  """
  @spec width(t()) :: float()
  def width(%__MODULE__{x0: x0, x1: x1}) do
    x1 - x0
  end

  @doc """
  Calculates the height of the bounding box.

  ## Parameters

    * `bbox` - A `BoundingBox` struct

  ## Returns

  The height (y1 - y0).

  ## Examples

      iex> bbox = %Kreuzberg.BoundingBox{x0: 10.0, y0: 20.0, x1: 110.0, y1: 70.0}
      iex> Kreuzberg.BoundingBox.height(bbox)
      50.0
  """
  @spec height(t()) :: float()
  def height(%__MODULE__{y0: y0, y1: y1}) do
    y1 - y0
  end

  defp to_float(value) when is_float(value), do: value
  defp to_float(value) when is_integer(value), do: value * 1.0
  defp to_float(nil), do: 0.0

  defp to_float(value) when is_binary(value) do
    case Float.parse(value) do
      {f, _} -> f
      :error -> 0.0
    end
  end
end

defmodule Kreuzberg.ElementMetadata do
  @moduledoc """
  Metadata for a semantic element extracted from a document.

  Contains information about where and how an element appears in the source
  document, including position, page number, and custom metadata fields.

  ## Fields

    * `:page_number` - Page number where element appears (1-indexed), or nil
    * `:filename` - Source filename or document name, or nil
    * `:coordinates` - Bounding box coordinates if available, or nil
    * `:element_index` - Position index in the element sequence, or nil
    * `:additional` - Map of custom metadata fields (defaults to empty map)

  ## Examples

      iex> metadata = %Kreuzberg.ElementMetadata{
      ...>   page_number: 1,
      ...>   filename: "document.pdf",
      ...>   coordinates: %Kreuzberg.BoundingBox{x0: 10.0, y0: 20.0, x1: 100.0, y1: 50.0},
      ...>   element_index: 0,
      ...>   additional: %{"section" => "Introduction"}
      ...> }
      iex> metadata.page_number
      1
  """

  @type t :: %__MODULE__{
          page_number: integer() | nil,
          filename: String.t() | nil,
          coordinates: Kreuzberg.BoundingBox.t() | nil,
          element_index: integer() | nil,
          additional: map()
        }

  defstruct [
    :page_number,
    :filename,
    :coordinates,
    :element_index,
    additional: %{}
  ]

  @doc """
  Creates an ElementMetadata struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct,
  handling nested bounding box data.

  ## Parameters

    * `data` - A map containing metadata fields

  ## Returns

  An `ElementMetadata` struct with properly typed fields.

  ## Examples

      iex> metadata_map = %{
      ...>   "page_number" => 1,
      ...>   "filename" => "document.pdf",
      ...>   "coordinates" => %{"x0" => 10.0, "y0" => 20.0, "x1" => 100.0, "y1" => 50.0},
      ...>   "element_index" => 0,
      ...>   "additional" => %{"section" => "Introduction"}
      ...> }
      iex> metadata = Kreuzberg.ElementMetadata.from_map(metadata_map)
      iex> metadata.page_number
      1
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    coordinates =
      case data["coordinates"] do
        nil -> nil
        %Kreuzberg.BoundingBox{} = bbox -> bbox
        map when is_map(map) -> Kreuzberg.BoundingBox.from_map(map)
        _ -> nil
      end

    %__MODULE__{
      page_number: data["page_number"],
      filename: data["filename"],
      coordinates: coordinates,
      element_index: data["element_index"],
      additional: data["additional"] || %{}
    }
  end

  @doc """
  Converts an ElementMetadata struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `metadata` - An `ElementMetadata` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> metadata = %Kreuzberg.ElementMetadata{
      ...>   page_number: 1,
      ...>   filename: "doc.pdf",
      ...>   additional: %{"section" => "intro"}
      ...> }
      iex> Kreuzberg.ElementMetadata.to_map(metadata)
      %{
        "page_number" => 1,
        "filename" => "doc.pdf",
        "coordinates" => nil,
        ...
      }
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = metadata) do
    %{
      "page_number" => metadata.page_number,
      "filename" => metadata.filename,
      "coordinates" =>
        case metadata.coordinates do
          nil -> nil
          bbox -> Kreuzberg.BoundingBox.to_map(bbox)
        end,
      "element_index" => metadata.element_index,
      "additional" => metadata.additional
    }
  end
end

defmodule Kreuzberg.Element do
  @moduledoc """
  Semantic element extracted from a document.

  Represents a logical unit of content with semantic classification,
  unique identifier, and metadata for tracking origin and position.
  Compatible with Unstructured.io element format.

  ## Element Types

  Elements are classified into semantic categories:

    * `:title` - Document title
    * `:narrative_text` - Main narrative text body
    * `:heading` - Section heading
    * `:list_item` - List item (bullet, numbered, etc.)
    * `:table` - Table element
    * `:image` - Image element
    * `:page_break` - Page break marker
    * `:code_block` - Code block
    * `:block_quote` - Block quote
    * `:footer` - Footer text
    * `:header` - Header text

  ## Fields

    * `:element_id` - Unique deterministic identifier for the element
    * `:element_type` - Semantic type classification (atom)
    * `:text` - Text content of the element
    * `:metadata` - ElementMetadata struct with position and source info

  ## Examples

      iex> element = %Kreuzberg.Element{
      ...>   element_id: "elem-12345",
      ...>   element_type: :narrative_text,
      ...>   text: "This is a paragraph of text.",
      ...>   metadata: %Kreuzberg.ElementMetadata{
      ...>     page_number: 1,
      ...>     filename: "document.pdf"
      ...>   }
      ...> }
      iex> element.element_type
      :narrative_text
      iex> element.text
      "This is a paragraph of text."
  """

  @typedoc "Semantic element type classification"
  @type element_type ::
          :title
          | :narrative_text
          | :heading
          | :list_item
          | :table
          | :image
          | :page_break
          | :code_block
          | :block_quote
          | :footer
          | :header

  @type t :: %__MODULE__{
          element_id: String.t(),
          element_type: element_type(),
          text: String.t(),
          metadata: Kreuzberg.ElementMetadata.t()
        }

  defstruct [:element_id, :element_type, :text, metadata: %Kreuzberg.ElementMetadata{}]

  @doc """
  Creates an Element struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct,
  handling nested metadata and type conversion.

  ## Parameters

    * `data` - A map containing element fields

  ## Returns

  An `Element` struct with properly typed fields.

  ## Examples

      iex> element_map = %{
      ...>   "element_id" => "elem-123",
      ...>   "element_type" => "narrative_text",
      ...>   "text" => "Paragraph content",
      ...>   "metadata" => %{"page_number" => 1}
      ...> }
      iex> element = Kreuzberg.Element.from_map(element_map)
      iex> element.element_type
      :narrative_text
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    element_type =
      case data["element_type"] do
        type when is_atom(type) -> type
        type when is_binary(type) -> string_to_atom(type)
        nil -> :narrative_text
        _ -> :narrative_text
      end

    metadata =
      case data["metadata"] do
        nil -> %Kreuzberg.ElementMetadata{}
        %Kreuzberg.ElementMetadata{} = m -> m
        map when is_map(map) -> Kreuzberg.ElementMetadata.from_map(map)
        _ -> %Kreuzberg.ElementMetadata{}
      end

    %__MODULE__{
      element_id: data["element_id"] || "",
      element_type: element_type,
      text: data["text"] || "",
      metadata: metadata
    }
  end

  @doc """
  Converts an Element struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `element` - An `Element` struct

  ## Returns

  A map with string keys representing all fields. Element type is converted to string.

  ## Examples

      iex> element = %Kreuzberg.Element{
      ...>   element_id: "elem-123",
      ...>   element_type: :narrative_text,
      ...>   text: "Content",
      ...>   metadata: %Kreuzberg.ElementMetadata{}
      ...> }
      iex> map = Kreuzberg.Element.to_map(element)
      iex> map["element_type"]
      "narrative_text"
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = element) do
    %{
      "element_id" => element.element_id,
      "element_type" => atom_to_string(element.element_type),
      "text" => element.text,
      "metadata" => Kreuzberg.ElementMetadata.to_map(element.metadata)
    }
  end

  @doc """
  Returns a human-readable description of the element type.

  ## Parameters

    * `element_type` - An element type atom

  ## Returns

  A string description of the element type.

  ## Examples

      iex> Kreuzberg.Element.type_description(:narrative_text)
      "Narrative text"

      iex> Kreuzberg.Element.type_description(:code_block)
      "Code block"
  """
  @spec type_description(element_type()) :: String.t()
  def type_description(:title), do: "Title"
  def type_description(:narrative_text), do: "Narrative text"
  def type_description(:heading), do: "Heading"
  def type_description(:list_item), do: "List item"
  def type_description(:table), do: "Table"
  def type_description(:image), do: "Image"
  def type_description(:page_break), do: "Page break"
  def type_description(:code_block), do: "Code block"
  def type_description(:block_quote), do: "Block quote"
  def type_description(:footer), do: "Footer"
  def type_description(:header), do: "Header"
  def type_description(_), do: "Unknown"

  defp string_to_atom("title"), do: :title
  defp string_to_atom("narrative_text"), do: :narrative_text
  defp string_to_atom("heading"), do: :heading
  defp string_to_atom("list_item"), do: :list_item
  defp string_to_atom("table"), do: :table
  defp string_to_atom("image"), do: :image
  defp string_to_atom("page_break"), do: :page_break
  defp string_to_atom("code_block"), do: :code_block
  defp string_to_atom("block_quote"), do: :block_quote
  defp string_to_atom("footer"), do: :footer
  defp string_to_atom("header"), do: :header
  defp string_to_atom(_), do: :narrative_text

  defp atom_to_string(:title), do: "title"
  defp atom_to_string(:narrative_text), do: "narrative_text"
  defp atom_to_string(:heading), do: "heading"
  defp atom_to_string(:list_item), do: "list_item"
  defp atom_to_string(:table), do: "table"
  defp atom_to_string(:image), do: "image"
  defp atom_to_string(:page_break), do: "page_break"
  defp atom_to_string(:code_block), do: "code_block"
  defp atom_to_string(:block_quote), do: "block_quote"
  defp atom_to_string(:footer), do: "footer"
  defp atom_to_string(:header), do: "header"
  defp atom_to_string(_), do: "narrative_text"
end

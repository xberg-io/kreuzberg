defmodule Kreuzberg.Image do
  @moduledoc """
  Structure representing an extracted image from a document.

  Matches the Rust `ExtractedImage` struct.

  ## Fields

    * `:data` - Raw binary image data (PNG, JPEG, WebP, etc.)
    * `:format` - Image format string ("png", "jpeg", "webp", etc.)
    * `:image_index` - Zero-indexed position of image in the document/page
    * `:page_number` - Page number where image was found
    * `:width` - Image width in pixels
    * `:height` - Image height in pixels
    * `:colorspace` - Color space (e.g., "RGB", "CMYK", "Gray")
    * `:bits_per_component` - Bits per color component (e.g., 8, 16)
    * `:is_mask` - Whether this image is a mask image
    * `:description` - Optional description of the image
    * `:ocr_result` - Nested OCR extraction result map
    * `:bounding_box` - Bounding box coordinates if available
  """

  @type t :: %__MODULE__{
          data: binary(),
          format: String.t(),
          image_index: non_neg_integer(),
          page_number: non_neg_integer() | nil,
          width: non_neg_integer() | nil,
          height: non_neg_integer() | nil,
          colorspace: String.t() | nil,
          bits_per_component: non_neg_integer() | nil,
          is_mask: boolean(),
          description: String.t() | nil,
          ocr_result: Kreuzberg.ExtractionResult.t() | nil,
          bounding_box: map() | nil
        }

  defstruct [
    :page_number,
    :width,
    :height,
    :colorspace,
    :bits_per_component,
    :description,
    :ocr_result,
    :bounding_box,
    data: <<>>,
    format: "",
    image_index: 0,
    is_mask: false
  ]

  @doc """
  Creates a new Image struct.

  ## Parameters

    * `format` - The image format (e.g., "png", "jpeg")
    * `opts` - Optional keyword list of additional fields
  """
  @spec new(String.t(), keyword()) :: t()
  def new(format, opts \\ []) when is_binary(format) do
    %__MODULE__{
      format: format,
      data: Keyword.get(opts, :data, <<>>),
      image_index: Keyword.get(opts, :image_index, 0),
      page_number: Keyword.get(opts, :page_number),
      width: Keyword.get(opts, :width),
      height: Keyword.get(opts, :height),
      colorspace: Keyword.get(opts, :colorspace),
      bits_per_component: Keyword.get(opts, :bits_per_component),
      is_mask: Keyword.get(opts, :is_mask, false),
      description: Keyword.get(opts, :description),
      ocr_result: Keyword.get(opts, :ocr_result)
    }
  end

  @doc """
  Creates an Image struct from a map.

  Handles `bytes::Bytes` serde serialization where binary data arrives
  as a list of u8 integers, converting it to Elixir binary.

  ## Examples

      iex> Kreuzberg.Image.from_map(%{"format" => "png", "image_index" => 0, "width" => 800})
      %Kreuzberg.Image{format: "png", image_index: 0, width: 800}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      data: normalize_image_data(data["data"]),
      format: data["format"] || "",
      image_index: data["image_index"] || 0,
      page_number: data["page_number"],
      width: data["width"],
      height: data["height"],
      colorspace: data["colorspace"],
      bits_per_component: data["bits_per_component"],
      is_mask: data["is_mask"] || false,
      description: data["description"],
      ocr_result: normalize_ocr_result(data["ocr_result"]),
      bounding_box: data["bounding_box"]
    }
  end

  @doc """
  Converts an Image struct to a map.
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = image) do
    %{
      "data" => image.data,
      "format" => image.format,
      "image_index" => image.image_index,
      "page_number" => image.page_number,
      "width" => image.width,
      "height" => image.height,
      "colorspace" => image.colorspace,
      "bits_per_component" => image.bits_per_component,
      "is_mask" => image.is_mask,
      "description" => image.description,
      "ocr_result" =>
        case image.ocr_result do
          nil -> nil
          %Kreuzberg.ExtractionResult{} = r -> Kreuzberg.ExtractionResult.to_map(r)
          other -> other
        end,
      "bounding_box" => image.bounding_box
    }
  end

  defp normalize_ocr_result(nil), do: nil
  defp normalize_ocr_result(%Kreuzberg.ExtractionResult{} = r), do: r

  defp normalize_ocr_result(map) when is_map(map) do
    Kreuzberg.ExtractionResult.new(
      map["content"] || "",
      map["mime_type"] || "",
      map["metadata"] || %{},
      map["tables"] || [],
      detected_languages: map["detected_languages"],
      chunks: map["chunks"],
      images: map["images"],
      pages: map["pages"],
      elements: map["elements"],
      djot_content: map["djot_content"],
      annotations: map["annotations"]
    )
  end

  # bytes::Bytes serializes via serde as an array of u8 integers.
  # Convert list of integers to binary for Elixir.
  defp normalize_image_data(nil), do: <<>>
  defp normalize_image_data(data) when is_binary(data), do: data
  defp normalize_image_data(data) when is_list(data), do: :binary.list_to_bin(data)

  @doc """
  Returns whether the image has binary data.
  """
  @spec has_data?(t()) :: boolean()
  def has_data?(%__MODULE__{data: data}) do
    is_binary(data) and byte_size(data) > 0
  end
end

defmodule Kreuzberg.OcrElement do
  @moduledoc """
  OCR-extracted text element with detailed positioning and confidence information.

  Represents a text element detected by OCR with its content, geometry,
  confidence scores, and hierarchical relationships within a document.

  ## Fields

    * `:text` - The recognized text content
    * `:geometry` - Bounding geometry for the element, or nil
    * `:confidence` - Confidence scores for detection and recognition, or nil
    * `:level` - Hierarchical level of the element (e.g., "page", "block", "line", "word"), or nil
    * `:rotation` - Rotation information for the text, or nil
    * `:page_number` - Page number where element appears (1-indexed), or nil
    * `:parent_id` - ID of parent element in hierarchy, or nil
    * `:backend_metadata` - Backend-specific metadata map, or nil

  ## Examples

      iex> element = %Kreuzberg.OcrElement{
      ...>   text: "Hello World",
      ...>   geometry: %Kreuzberg.OcrBoundingGeometry{type: "rect", left: 10.0, top: 20.0, width: 100.0, height: 30.0},
      ...>   confidence: %Kreuzberg.OcrConfidence{detection: 0.95, recognition: 0.92},
      ...>   level: "line",
      ...>   page_number: 1
      ...> }
      iex> element.text
      "Hello World"
  """

  @type t :: %__MODULE__{
          text: String.t(),
          geometry: Kreuzberg.OcrBoundingGeometry.t() | nil,
          confidence: Kreuzberg.OcrConfidence.t() | nil,
          level: String.t() | nil,
          rotation: Kreuzberg.OcrRotation.t() | nil,
          page_number: integer() | nil,
          parent_id: String.t() | nil,
          backend_metadata: map() | nil
        }

  defstruct [
    :geometry,
    :confidence,
    :level,
    :rotation,
    :page_number,
    :parent_id,
    :backend_metadata,
    text: ""
  ]

  @doc """
  Creates an OcrElement struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct,
  handling nested geometry, confidence, and rotation data.

  ## Parameters

    * `data` - A map containing OCR element fields

  ## Returns

  An `OcrElement` struct with properly typed fields.

  ## Examples

      iex> element_map = %{
      ...>   "text" => "Hello World",
      ...>   "geometry" => %{"type" => "rect", "left" => 10.0, "top" => 20.0, "width" => 100.0, "height" => 30.0},
      ...>   "confidence" => %{"detection" => 0.95, "recognition" => 0.92},
      ...>   "level" => "line",
      ...>   "page_number" => 1
      ...> }
      iex> element = Kreuzberg.OcrElement.from_map(element_map)
      iex> element.text
      "Hello World"
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    geometry =
      case data["geometry"] do
        nil -> nil
        %Kreuzberg.OcrBoundingGeometry{} = g -> g
        map when is_map(map) -> Kreuzberg.OcrBoundingGeometry.from_map(map)
        _ -> nil
      end

    confidence =
      case data["confidence"] do
        nil -> nil
        %Kreuzberg.OcrConfidence{} = c -> c
        map when is_map(map) -> Kreuzberg.OcrConfidence.from_map(map)
        _ -> nil
      end

    rotation =
      case data["rotation"] do
        nil -> nil
        %Kreuzberg.OcrRotation{} = r -> r
        map when is_map(map) -> Kreuzberg.OcrRotation.from_map(map)
        _ -> nil
      end

    %__MODULE__{
      text: data["text"] || "",
      geometry: geometry,
      confidence: confidence,
      level: data["level"],
      rotation: rotation,
      page_number: data["page_number"],
      parent_id: data["parent_id"],
      backend_metadata: data["backend_metadata"]
    }
  end

  @doc """
  Converts an OcrElement struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `element` - An `OcrElement` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> element = %Kreuzberg.OcrElement{
      ...>   text: "Hello World",
      ...>   geometry: %Kreuzberg.OcrBoundingGeometry{type: "rect", left: 10.0, top: 20.0, width: 100.0, height: 30.0},
      ...>   confidence: %Kreuzberg.OcrConfidence{detection: 0.95, recognition: 0.92},
      ...>   level: "line",
      ...>   page_number: 1
      ...> }
      iex> map = Kreuzberg.OcrElement.to_map(element)
      iex> map["text"]
      "Hello World"
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = element) do
    %{
      "text" => element.text,
      "geometry" =>
        case element.geometry do
          nil -> nil
          g -> Kreuzberg.OcrBoundingGeometry.to_map(g)
        end,
      "confidence" =>
        case element.confidence do
          nil -> nil
          c -> Kreuzberg.OcrConfidence.to_map(c)
        end,
      "level" => element.level,
      "rotation" =>
        case element.rotation do
          nil -> nil
          r -> Kreuzberg.OcrRotation.to_map(r)
        end,
      "page_number" => element.page_number,
      "parent_id" => element.parent_id,
      "backend_metadata" => element.backend_metadata
    }
  end
end

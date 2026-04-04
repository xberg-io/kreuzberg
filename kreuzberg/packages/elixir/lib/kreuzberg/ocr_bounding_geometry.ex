defmodule Kreuzberg.OcrBoundingGeometry do
  @moduledoc """
  Bounding geometry for OCR-extracted text elements.

  Represents the geometric positioning of OCR-detected text,
  supporting both rectangular (left/top/width/height) and point-based
  (polygon points) geometry definitions.

  ## Fields

    * `:type` - Geometry type ("rect" or "polygon")
    * `:left` - Left x-coordinate for rectangular geometry, or nil
    * `:top` - Top y-coordinate for rectangular geometry, or nil
    * `:width` - Width for rectangular geometry, or nil
    * `:height` - Height for rectangular geometry, or nil
    * `:points` - Array of [x, y] points for polygon geometry, or nil

  ## Examples

      iex> geometry = %Kreuzberg.OcrBoundingGeometry{
      ...>   type: "rect",
      ...>   left: 10.0,
      ...>   top: 20.0,
      ...>   width: 100.0,
      ...>   height: 50.0
      ...> }
      iex> geometry.type
      "rect"
  """

  @type t :: %__MODULE__{
          type: String.t(),
          left: float() | nil,
          top: float() | nil,
          width: float() | nil,
          height: float() | nil,
          points: list(list(float())) | nil
        }

  defstruct [
    :left,
    :top,
    :width,
    :height,
    :points,
    type: "rect"
  ]

  @doc """
  Creates an OcrBoundingGeometry struct from a map.

  ## Parameters

    * `data` - A map containing geometry fields

  ## Returns

  An `OcrBoundingGeometry` struct with properly typed fields.

  ## Examples

      iex> geometry_map = %{
      ...>   "type" => "rect",
      ...>   "left" => 10.0,
      ...>   "top" => 20.0,
      ...>   "width" => 100.0,
      ...>   "height" => 50.0
      ...> }
      iex> geometry = Kreuzberg.OcrBoundingGeometry.from_map(geometry_map)
      iex> geometry.type
      "rect"
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      type: data["type"] || "rect",
      left: to_float(data["left"]),
      top: to_float(data["top"]),
      width: to_float(data["width"]),
      height: to_float(data["height"]),
      points: normalize_points(data["points"])
    }
  end

  @doc """
  Converts an OcrBoundingGeometry struct to a map.

  ## Parameters

    * `geometry` - An `OcrBoundingGeometry` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> geometry = %Kreuzberg.OcrBoundingGeometry{
      ...>   type: "rect",
      ...>   left: 10.0,
      ...>   top: 20.0,
      ...>   width: 100.0,
      ...>   height: 50.0
      ...> }
      iex> Kreuzberg.OcrBoundingGeometry.to_map(geometry)
      %{
        "type" => "rect",
        "left" => 10.0,
        ...
      }
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = geometry) do
    %{
      "type" => geometry.type,
      "left" => geometry.left,
      "top" => geometry.top,
      "width" => geometry.width,
      "height" => geometry.height,
      "points" => geometry.points
    }
  end

  defp to_float(value) when is_float(value), do: value
  defp to_float(value) when is_integer(value), do: value * 1.0
  defp to_float(nil), do: nil

  defp to_float(value) when is_binary(value) do
    case Float.parse(value) do
      {f, _} -> f
      :error -> nil
    end
  end

  defp to_float(_), do: nil

  defp normalize_points(nil), do: nil

  defp normalize_points(points) when is_list(points) do
    Enum.map(points, fn
      [x, y] ->
        [to_float(x), to_float(y)]

      point when is_list(point) and length(point) >= 2 ->
        [to_float(Enum.at(point, 0)), to_float(Enum.at(point, 1))]

      _ ->
        [0.0, 0.0]
    end)
  end

  defp normalize_points(_), do: nil
end

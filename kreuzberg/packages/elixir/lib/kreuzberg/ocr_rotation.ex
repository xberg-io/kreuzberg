defmodule Kreuzberg.OcrRotation do
  @moduledoc """
  Rotation information for OCR-detected text.

  Represents the rotation angle of text as detected by OCR,
  including the confidence of the angle detection.

  ## Fields

    * `:angle_degrees` - Rotation angle in degrees (-180 to 180), or nil
    * `:confidence` - Confidence score for rotation detection (0.0-1.0), or nil

  ## Examples

      iex> rotation = %Kreuzberg.OcrRotation{
      ...>   angle_degrees: 15.5,
      ...>   confidence: 0.88
      ...> }
      iex> rotation.angle_degrees
      15.5
  """

  @type t :: %__MODULE__{
          angle_degrees: float() | nil,
          confidence: float() | nil
        }

  defstruct [:angle_degrees, :confidence]

  @doc """
  Creates an OcrRotation struct from a map.

  ## Parameters

    * `data` - A map containing rotation fields

  ## Returns

  An `OcrRotation` struct with properly typed fields.

  ## Examples

      iex> rotation_map = %{"angle_degrees" => 15.5, "confidence" => 0.88}
      iex> rotation = Kreuzberg.OcrRotation.from_map(rotation_map)
      iex> rotation.angle_degrees
      15.5
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      angle_degrees: to_float(data["angle_degrees"]),
      confidence: to_float(data["confidence"])
    }
  end

  @doc """
  Converts an OcrRotation struct to a map.

  ## Parameters

    * `rotation` - An `OcrRotation` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> rotation = %Kreuzberg.OcrRotation{
      ...>   angle_degrees: 15.5,
      ...>   confidence: 0.88
      ...> }
      iex> Kreuzberg.OcrRotation.to_map(rotation)
      %{"angle_degrees" => 15.5, "confidence" => 0.88}
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = rotation) do
    %{
      "angle_degrees" => rotation.angle_degrees,
      "confidence" => rotation.confidence
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
end

defmodule Kreuzberg.OcrConfidence do
  @moduledoc """
  Confidence scores for OCR text detection and recognition.

  Contains separate confidence metrics for different stages of the OCR process:
  detection (finding text regions) and recognition (reading the text).

  ## Fields

    * `:detection` - Confidence score for text detection (0.0-1.0), or nil
    * `:recognition` - Confidence score for text recognition (0.0-1.0), or nil

  ## Examples

      iex> confidence = %Kreuzberg.OcrConfidence{
      ...>   detection: 0.95,
      ...>   recognition: 0.92
      ...> }
      iex> confidence.detection
      0.95
  """

  @type t :: %__MODULE__{
          detection: float() | nil,
          recognition: float() | nil
        }

  defstruct [:detection, :recognition]

  @doc """
  Creates an OcrConfidence struct from a map.

  ## Parameters

    * `data` - A map containing confidence fields

  ## Returns

  An `OcrConfidence` struct with properly typed fields.

  ## Examples

      iex> confidence_map = %{"detection" => 0.95, "recognition" => 0.92}
      iex> confidence = Kreuzberg.OcrConfidence.from_map(confidence_map)
      iex> confidence.detection
      0.95
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      detection: to_float(data["detection"]),
      recognition: to_float(data["recognition"])
    }
  end

  @doc """
  Converts an OcrConfidence struct to a map.

  ## Parameters

    * `confidence` - An `OcrConfidence` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> confidence = %Kreuzberg.OcrConfidence{
      ...>   detection: 0.95,
      ...>   recognition: 0.92
      ...> }
      iex> Kreuzberg.OcrConfidence.to_map(confidence)
      %{"detection" => 0.95, "recognition" => 0.92}
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = confidence) do
    %{
      "detection" => confidence.detection,
      "recognition" => confidence.recognition
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

defmodule Kreuzberg.ProcessingWarning do
  @moduledoc """
  Structure representing a warning generated during document processing.

  Matches the Rust `ProcessingWarning` struct.

  ## Fields

    * `:source` - The component or stage that generated the warning
    * `:message` - Human-readable warning message
  """

  @type t :: %__MODULE__{
          source: String.t(),
          message: String.t()
        }

  defstruct source: "", message: ""

  @doc """
  Create a ProcessingWarning struct from a map.

  ## Examples

      iex> Kreuzberg.ProcessingWarning.from_map(%{
      ...>   "source" => "ocr",
      ...>   "message" => "Low confidence detected"
      ...> })
      %Kreuzberg.ProcessingWarning{
        source: "ocr",
        message: "Low confidence detected"
      }
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      source: data["source"] || "",
      message: data["message"] || ""
    }
  end

  @doc """
  Convert a ProcessingWarning struct to a map.

  ## Examples

      iex> warning = %Kreuzberg.ProcessingWarning{source: "ocr", message: "Low confidence"}
      iex> Kreuzberg.ProcessingWarning.to_map(warning)
      %{
        "source" => "ocr",
        "message" => "Low confidence"
      }
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = warning) do
    %{
      "source" => warning.source,
      "message" => warning.message
    }
  end
end

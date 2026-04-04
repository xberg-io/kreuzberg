defmodule Kreuzberg.ErrorMetadata do
  @moduledoc """
  Error metadata when extraction partially failed.

  Matches the Rust `ErrorMetadata` struct.

  ## Fields

    * `:error_type` - The type/category of the error
    * `:message` - Human-readable error message
  """

  @type t :: %__MODULE__{
          error_type: String.t(),
          message: String.t()
        }

  defstruct error_type: "", message: ""

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      error_type: data["error_type"] || "",
      message: data["message"] || ""
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = err) do
    %{
      "error_type" => err.error_type,
      "message" => err.message
    }
  end
end

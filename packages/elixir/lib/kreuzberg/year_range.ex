defmodule Kreuzberg.YearRange do
  @moduledoc """
  Year range for bibliographic metadata.

  Matches the Rust `YearRange` struct.

  ## Fields

    * `:min` - Minimum year
    * `:max` - Maximum year
    * `:years` - List of individual years
  """

  @type t :: %__MODULE__{
          min: non_neg_integer() | nil,
          max: non_neg_integer() | nil,
          years: list(non_neg_integer())
        }

  defstruct min: nil,
            max: nil,
            years: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      min: data["min"],
      max: data["max"],
      years: data["years"] || []
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "min" => meta.min,
      "max" => meta.max,
      "years" => meta.years
    }
  end
end

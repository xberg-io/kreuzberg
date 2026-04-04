defmodule Kreuzberg.FictionBookMetadata do
  @moduledoc """
  FictionBook (FB2) metadata.

  Matches the Rust `FictionBookMetadata` struct.

  ## Fields

    * `:genres` - List of genres
    * `:sequences` - List of sequences/series
    * `:annotation` - Book annotation/description
  """

  @type t :: %__MODULE__{
          genres: list(String.t()),
          sequences: list(String.t()),
          annotation: String.t() | nil
        }

  defstruct genres: [],
            sequences: [],
            annotation: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      genres: data["genres"] || [],
      sequences: data["sequences"] || [],
      annotation: data["annotation"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "genres" => meta.genres,
      "sequences" => meta.sequences,
      "annotation" => meta.annotation
    }
  end
end

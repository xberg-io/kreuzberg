defmodule Kreuzberg.EpubMetadata do
  @moduledoc """
  EPUB metadata (Dublin Core extensions).

  Matches the Rust `EpubMetadata` struct.

  ## Fields

    * `:coverage` - Geographic/temporal coverage
    * `:dc_format` - Dublin Core format
    * `:relation` - Related resource
    * `:source` - Source resource
    * `:dc_type` - Dublin Core type
    * `:cover_image` - Cover image path
  """

  @type t :: %__MODULE__{
          coverage: String.t() | nil,
          dc_format: String.t() | nil,
          relation: String.t() | nil,
          source: String.t() | nil,
          dc_type: String.t() | nil,
          cover_image: String.t() | nil
        }

  defstruct coverage: nil,
            dc_format: nil,
            relation: nil,
            source: nil,
            dc_type: nil,
            cover_image: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      coverage: data["coverage"],
      dc_format: data["dc_format"],
      relation: data["relation"],
      source: data["source"],
      dc_type: data["dc_type"],
      cover_image: data["cover_image"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "coverage" => meta.coverage,
      "dc_format" => meta.dc_format,
      "relation" => meta.relation,
      "source" => meta.source,
      "dc_type" => meta.dc_type,
      "cover_image" => meta.cover_image
    }
  end
end

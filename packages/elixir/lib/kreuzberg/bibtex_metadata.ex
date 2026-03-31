defmodule Kreuzberg.BibtexMetadata do
  @moduledoc """
  BibTeX bibliography metadata.

  Matches the Rust `BibtexMetadata` struct.

  ## Fields

    * `:entry_count` - Number of BibTeX entries
    * `:citation_keys` - List of citation keys
    * `:authors` - List of author names
    * `:year_range` - Year range of entries
    * `:entry_types` - Map of entry type to count
  """

  @type t :: %__MODULE__{
          entry_count: non_neg_integer(),
          citation_keys: list(String.t()),
          authors: list(String.t()),
          year_range: Kreuzberg.YearRange.t() | nil,
          entry_types: map() | nil
        }

  defstruct entry_count: 0,
            citation_keys: [],
            authors: [],
            year_range: nil,
            entry_types: nil

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      entry_count: data["entry_count"] || 0,
      citation_keys: data["citation_keys"] || [],
      authors: data["authors"] || [],
      year_range: normalize_year_range(data["year_range"]),
      entry_types: data["entry_types"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "entry_count" => meta.entry_count,
      "citation_keys" => meta.citation_keys,
      "authors" => meta.authors,
      "year_range" => serialize_year_range(meta.year_range),
      "entry_types" => meta.entry_types
    }
  end

  defp normalize_year_range(nil), do: nil
  defp normalize_year_range(%Kreuzberg.YearRange{} = yr), do: yr
  defp normalize_year_range(map) when is_map(map), do: Kreuzberg.YearRange.from_map(map)
  defp normalize_year_range(_), do: nil

  defp serialize_year_range(nil), do: nil
  defp serialize_year_range(%Kreuzberg.YearRange{} = yr), do: Kreuzberg.YearRange.to_map(yr)
  defp serialize_year_range(other), do: other
end

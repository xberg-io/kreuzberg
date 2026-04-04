defmodule Kreuzberg.CitationMetadata do
  @moduledoc """
  Citation file metadata (RIS, PubMed, EndNote).

  Matches the Rust `CitationMetadata` struct.

  ## Fields

    * `:citation_count` - Number of citations
    * `:format` - Citation format (e.g., "RIS", "PubMed")
    * `:authors` - List of author names
    * `:year_range` - Year range of citations
    * `:dois` - List of DOIs
    * `:keywords` - List of keywords
  """

  @type t :: %__MODULE__{
          citation_count: non_neg_integer(),
          format: String.t() | nil,
          authors: list(String.t()),
          year_range: Kreuzberg.YearRange.t() | nil,
          dois: list(String.t()),
          keywords: list(String.t())
        }

  defstruct citation_count: 0,
            format: nil,
            authors: [],
            year_range: nil,
            dois: [],
            keywords: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      citation_count: data["citation_count"] || 0,
      format: data["format"],
      authors: data["authors"] || [],
      year_range: normalize_year_range(data["year_range"]),
      dois: data["dois"] || [],
      keywords: data["keywords"] || []
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = meta) do
    %{
      "citation_count" => meta.citation_count,
      "format" => meta.format,
      "authors" => meta.authors,
      "year_range" => serialize_year_range(meta.year_range),
      "dois" => meta.dois,
      "keywords" => meta.keywords
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

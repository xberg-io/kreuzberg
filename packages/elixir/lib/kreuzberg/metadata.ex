defmodule Kreuzberg.Metadata do
  @moduledoc """
  Structure representing document metadata extracted from files.

  Matches the Rust `Metadata` struct. Note that `format` and `additional`
  use `#[serde(flatten)]` in Rust, so their fields appear at the root level
  of the serialized JSON.

  ## Fields

    * `:title` - Document title
    * `:subject` - Document subject or description
    * `:authors` - List of author names
    * `:keywords` - List of keywords
    * `:language` - Primary language (ISO 639-1 code)
    * `:created_at` - Creation date (ISO 8601)
    * `:modified_at` - Last modification date (ISO 8601)
    * `:created_by` - Application that created the document
    * `:modified_by` - Application that last modified the document
    * `:pages` - Page structure information
    * `:format` - Format-specific metadata (flattened from Rust)
    * `:image_preprocessing` - Image preprocessing metadata
    * `:json_schema` - JSON schema if applicable
    * `:error` - Error metadata if extraction partially failed
    * `:category` - Document category classification
    * `:tags` - List of document tags
    * `:document_version` - Version of the document
    * `:abstract_text` - Abstract or summary of the document
    * `:output_format` - Output format used for extraction
    * `:additional` - Additional metadata fields (flattened from Rust)
  """

  # Known top-level metadata keys (non-format, non-additional)
  @known_keys MapSet.new([
    "title", "subject", "authors", "keywords", "language",
    "created_at", "modified_at", "created_by", "modified_by",
    "pages", "image_preprocessing", "json_schema", "error",
    "category", "tags", "document_version", "abstract_text", "output_format",
    "extraction_duration_ms"
  ])

  @type t :: %__MODULE__{
          title: String.t() | nil,
          subject: String.t() | nil,
          authors: list(String.t()) | nil,
          keywords: list(String.t()) | nil,
          language: String.t() | nil,
          created_at: String.t() | nil,
          modified_at: String.t() | nil,
          created_by: String.t() | nil,
          modified_by: String.t() | nil,
          pages: Kreuzberg.PageStructure.t() | nil,
          format: map() | nil,
          image_preprocessing: Kreuzberg.ImagePreprocessingMetadata.t() | nil,
          json_schema: map() | nil,
          error: Kreuzberg.ErrorMetadata.t() | nil,
          category: String.t() | nil,
          tags: list(String.t()) | nil,
          document_version: String.t() | nil,
          abstract_text: String.t() | nil,
          output_format: String.t() | nil,
          extraction_duration_ms: non_neg_integer() | nil,
          additional: map()
        }

  defstruct [
    :title,
    :subject,
    :authors,
    :keywords,
    :language,
    :created_at,
    :modified_at,
    :created_by,
    :modified_by,
    :pages,
    :format,
    :image_preprocessing,
    :json_schema,
    :error,
    :category,
    :tags,
    :document_version,
    :abstract_text,
    :output_format,
    :extraction_duration_ms,
    additional: %{}
  ]

  @doc """
  Creates a Metadata struct from a map.

  Handles Rust's `#[serde(flatten)]` by classifying keys into format fields,
  known metadata fields, and additional (catch-all) fields.

  ## Examples

      iex> Kreuzberg.Metadata.from_map(%{"title" => "Report", "authors" => ["Alice"]})
      %Kreuzberg.Metadata{title: "Report", authors: ["Alice"]}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    # Collect all keys that aren't known top-level metadata fields.
    # If "format_type" is present, these are flattened FormatMetadata fields → format.
    # If "format_type" is absent, they are additional catch-all fields → additional.
    has_format = Map.has_key?(data, "format_type")

    extra =
      Enum.reduce(data, %{}, fn {key, value}, acc ->
        if MapSet.member?(@known_keys, key), do: acc, else: Map.put(acc, key, value)
      end)

    {format, additional_map} =
      if has_format and map_size(extra) > 0 do
        {extra, %{}}
      else
        {nil, extra}
      end

    %__MODULE__{
      title: data["title"],
      subject: data["subject"],
      authors: data["authors"],
      keywords: data["keywords"],
      language: data["language"],
      created_at: data["created_at"],
      modified_at: data["modified_at"],
      created_by: data["created_by"],
      modified_by: data["modified_by"],
      pages: normalize_pages(data["pages"]),
      format: format,
      image_preprocessing: normalize_image_preprocessing(data["image_preprocessing"]),
      json_schema: data["json_schema"],
      error: normalize_error(data["error"]),
      category: data["category"],
      tags: data["tags"],
      document_version: data["document_version"],
      abstract_text: data["abstract_text"],
      output_format: data["output_format"],
      extraction_duration_ms: data["extraction_duration_ms"],
      additional: additional_map
    }
  end

  @doc """
  Converts a Metadata struct to a map.

  Re-flattens `format` and `additional` back into the root map to match
  the Rust serialization format.

  ## Examples

      iex> meta = %Kreuzberg.Metadata{title: "Report", format: %{"format_type" => "pdf"}}
      iex> map = Kreuzberg.Metadata.to_map(meta)
      iex> map["title"]
      "Report"
      iex> map["format_type"]
      "pdf"
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = metadata) do
    base = %{
      "title" => metadata.title,
      "subject" => metadata.subject,
      "authors" => metadata.authors,
      "keywords" => metadata.keywords,
      "language" => metadata.language,
      "created_at" => metadata.created_at,
      "modified_at" => metadata.modified_at,
      "created_by" => metadata.created_by,
      "modified_by" => metadata.modified_by,
      "pages" => serialize_pages(metadata.pages),
      "image_preprocessing" => serialize_image_preprocessing(metadata.image_preprocessing),
      "json_schema" => metadata.json_schema,
      "error" => serialize_error(metadata.error),
      "category" => metadata.category,
      "tags" => metadata.tags,
      "document_version" => metadata.document_version,
      "abstract_text" => metadata.abstract_text,
      "output_format" => metadata.output_format,
      "extraction_duration_ms" => metadata.extraction_duration_ms
    }

    # Re-flatten format and additional into root
    base
    |> maybe_merge(metadata.format)
    |> maybe_merge(metadata.additional)
  end

  defp maybe_merge(map, nil), do: map
  defp maybe_merge(map, other) when map_size(other) == 0, do: map
  defp maybe_merge(map, other), do: Map.merge(map, other)

  defp normalize_pages(nil), do: nil
  defp normalize_pages(%Kreuzberg.PageStructure{} = ps), do: ps
  defp normalize_pages(map) when is_map(map), do: Kreuzberg.PageStructure.from_map(map)
  defp normalize_pages(_other), do: nil

  defp normalize_image_preprocessing(nil), do: nil
  defp normalize_image_preprocessing(%Kreuzberg.ImagePreprocessingMetadata{} = m), do: m
  defp normalize_image_preprocessing(map) when is_map(map), do: Kreuzberg.ImagePreprocessingMetadata.from_map(map)

  defp normalize_error(nil), do: nil
  defp normalize_error(%Kreuzberg.ErrorMetadata{} = e), do: e
  defp normalize_error(map) when is_map(map), do: Kreuzberg.ErrorMetadata.from_map(map)

  defp serialize_pages(nil), do: nil
  defp serialize_pages(%Kreuzberg.PageStructure{} = ps), do: Kreuzberg.PageStructure.to_map(ps)
  defp serialize_pages(other), do: other

  defp serialize_image_preprocessing(nil), do: nil
  defp serialize_image_preprocessing(%Kreuzberg.ImagePreprocessingMetadata{} = m), do: Kreuzberg.ImagePreprocessingMetadata.to_map(m)
  defp serialize_image_preprocessing(other), do: other

  defp serialize_error(nil), do: nil
  defp serialize_error(%Kreuzberg.ErrorMetadata{} = e), do: Kreuzberg.ErrorMetadata.to_map(e)
  defp serialize_error(other), do: other
end

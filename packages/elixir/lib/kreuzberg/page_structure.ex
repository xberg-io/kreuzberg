defmodule Kreuzberg.PageBoundary do
  @moduledoc """
  Byte offset boundary for a page.

  Tracks where a specific page's content starts and ends in the main content string.
  Matches the Rust `PageBoundary` struct.

  ## Fields

    * `:byte_start` - Byte offset where this page starts (inclusive)
    * `:byte_end` - Byte offset where this page ends (exclusive)
    * `:page_number` - Page number (1-indexed)
  """

  @derive Jason.Encoder

  @type t :: %__MODULE__{
          byte_start: non_neg_integer(),
          byte_end: non_neg_integer(),
          page_number: non_neg_integer()
        }

  defstruct byte_start: 0, byte_end: 0, page_number: 0

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      byte_start: data["byte_start"] || 0,
      byte_end: data["byte_end"] || 0,
      page_number: data["page_number"] || 0
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = b) do
    %{
      "byte_start" => b.byte_start,
      "byte_end" => b.byte_end,
      "page_number" => b.page_number
    }
  end
end

defmodule Kreuzberg.PageInfo do
  @moduledoc """
  Metadata for an individual page/slide/sheet.

  Matches the Rust `PageInfo` struct.

  ## Fields

    * `:number` - Page number (1-indexed)
    * `:title` - Page title (usually for presentations)
    * `:dimensions` - Dimensions as `{width, height}` or nil
    * `:image_count` - Number of images on this page
    * `:table_count` - Number of tables on this page
    * `:hidden` - Whether this page is hidden
  """

  @derive Jason.Encoder

  @type t :: %__MODULE__{
          number: non_neg_integer(),
          title: String.t() | nil,
          dimensions: {float(), float()} | nil,
          image_count: non_neg_integer() | nil,
          table_count: non_neg_integer() | nil,
          hidden: boolean() | nil
        }

  defstruct [
    :title,
    :dimensions,
    :image_count,
    :table_count,
    :hidden,
    number: 0
  ]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      number: data["number"] || 0,
      title: data["title"],
      dimensions: normalize_dimensions(data["dimensions"]),
      image_count: data["image_count"],
      table_count: data["table_count"],
      hidden: data["hidden"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = info) do
    %{
      "number" => info.number,
      "title" => info.title,
      "dimensions" =>
        case info.dimensions do
          nil -> nil
          {w, h} -> [w, h]
        end,
      "image_count" => info.image_count,
      "table_count" => info.table_count,
      "hidden" => info.hidden
    }
  end

  defp normalize_dimensions(nil), do: nil
  defp normalize_dimensions([w, h]), do: {w * 1.0, h * 1.0}
  defp normalize_dimensions({w, h}), do: {w * 1.0, h * 1.0}
  defp normalize_dimensions(_), do: nil
end

defmodule Kreuzberg.PageStructure do
  @moduledoc """
  Page structure information for a document.

  Matches the Rust `PageStructure` struct.

  ## Fields

    * `:total_count` - Total number of pages/slides/sheets
    * `:unit_type` - Type of paginated unit ("page", "slide", "sheet")
    * `:boundaries` - Optional list of byte offset boundaries per page
    * `:pages` - Optional list of per-page metadata
  """

  @derive Jason.Encoder

  @type t :: %__MODULE__{
          total_count: non_neg_integer(),
          unit_type: String.t(),
          boundaries: list(Kreuzberg.PageBoundary.t()) | nil,
          pages: list(Kreuzberg.PageInfo.t()) | nil
        }

  defstruct [
    :boundaries,
    :pages,
    total_count: 0,
    unit_type: "page"
  ]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      total_count: data["total_count"] || 0,
      unit_type: data["unit_type"] || "page",
      boundaries: normalize_list(data["boundaries"], &Kreuzberg.PageBoundary.from_map/1),
      pages: normalize_list(data["pages"], &Kreuzberg.PageInfo.from_map/1)
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = ps) do
    %{
      "total_count" => ps.total_count,
      "unit_type" => ps.unit_type,
      "boundaries" => maybe_map(ps.boundaries, &Kreuzberg.PageBoundary.to_map/1),
      "pages" => maybe_map(ps.pages, &Kreuzberg.PageInfo.to_map/1)
    }
  end

  defp normalize_list(nil, _fun), do: nil
  defp normalize_list(list, fun) when is_list(list), do: Enum.map(list, fun)

  defp maybe_map(nil, _fun), do: nil
  defp maybe_map(list, fun) when is_list(list), do: Enum.map(list, fun)
end

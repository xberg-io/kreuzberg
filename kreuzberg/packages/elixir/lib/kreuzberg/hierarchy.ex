defmodule Kreuzberg.HierarchicalBlock do
  @moduledoc """
  A hierarchical block within a page, representing heading-level structure.

  Matches the Rust `HierarchicalBlock` struct.

  ## Fields

    * `:text` - The text content of this block
    * `:font_size` - The font size of the text
    * `:level` - Hierarchy level ("h1"-"h6" or "body")
    * `:bbox` - Optional bounding box as [left, top, right, bottom]
  """

  @type t :: %__MODULE__{
          text: String.t(),
          font_size: float(),
          level: String.t(),
          bbox: list(float()) | nil
        }

  defstruct [
    :bbox,
    text: "",
    font_size: 0.0,
    level: "body"
  ]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      text: data["text"] || "",
      font_size: (data["font_size"] || 0.0) * 1.0,
      level: data["level"] || "body",
      bbox: data["bbox"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = block) do
    %{
      "text" => block.text,
      "font_size" => block.font_size,
      "level" => block.level,
      "bbox" => block.bbox
    }
  end
end

defmodule Kreuzberg.PageHierarchy do
  @moduledoc """
  Hierarchy information for a page, containing heading-level blocks.

  Matches the Rust `PageHierarchy` struct.

  ## Fields

    * `:block_count` - Number of hierarchy blocks on this page
    * `:blocks` - List of hierarchical blocks
  """

  @type t :: %__MODULE__{
          block_count: non_neg_integer(),
          blocks: list(Kreuzberg.HierarchicalBlock.t())
        }

  defstruct block_count: 0, blocks: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    blocks =
      case data["blocks"] do
        nil ->
          []

        list when is_list(list) ->
          Enum.map(list, fn
            %Kreuzberg.HierarchicalBlock{} = b -> b
            map when is_map(map) -> Kreuzberg.HierarchicalBlock.from_map(map)
          end)
      end

    %__MODULE__{
      block_count: data["block_count"] || 0,
      blocks: blocks
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = h) do
    %{
      "block_count" => h.block_count,
      "blocks" => Enum.map(h.blocks, &Kreuzberg.HierarchicalBlock.to_map/1)
    }
  end
end

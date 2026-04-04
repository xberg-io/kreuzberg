defmodule Kreuzberg.DjotAttributes do
  @moduledoc """
  Element attributes in Djot ({.class #id key="value"} syntax).

  Matches the Rust `Attributes` struct.
  """

  @type t :: %__MODULE__{
          id: String.t() | nil,
          classes: list(String.t()),
          key_values: list({String.t(), String.t()})
        }

  defstruct [:id, classes: [], key_values: []]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      id: data["id"],
      classes: data["classes"] || [],
      key_values: normalize_key_values(data["key_values"])
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = attrs) do
    %{
      "id" => attrs.id,
      "classes" => attrs.classes,
      "key_values" => attrs.key_values
    }
  end

  # serde serializes Vec<(String, String)> as [[k, v], ...]
  defp normalize_key_values(nil), do: []

  defp normalize_key_values(list) when is_list(list) do
    Enum.map(list, fn
      [k, v] -> {k, v}
      {k, v} -> {k, v}
      other -> other
    end)
  end
end

defmodule Kreuzberg.DjotInlineElement do
  @moduledoc """
  Inline element within a Djot block (text, emphasis, link, etc.).

  Matches the Rust `InlineElement` struct.
  """

  @type t :: %__MODULE__{
          element_type: String.t(),
          content: String.t(),
          attributes: Kreuzberg.DjotAttributes.t() | nil,
          metadata: map() | nil
        }

  defstruct [:attributes, :metadata, element_type: "text", content: ""]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      element_type: data["element_type"] || "text",
      content: data["content"] || "",
      attributes: maybe_attrs(data["attributes"]),
      metadata: data["metadata"]
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = el) do
    %{
      "element_type" => el.element_type,
      "content" => el.content,
      "attributes" =>
        case el.attributes do
          nil -> nil
          a -> Kreuzberg.DjotAttributes.to_map(a)
        end,
      "metadata" => el.metadata
    }
  end

  defp maybe_attrs(nil), do: nil
  defp maybe_attrs(%Kreuzberg.DjotAttributes{} = a), do: a
  defp maybe_attrs(map) when is_map(map), do: Kreuzberg.DjotAttributes.from_map(map)
end

defmodule Kreuzberg.DjotFormattedBlock do
  @moduledoc """
  Block-level element in a Djot document (paragraph, heading, list, etc.).

  Matches the Rust `FormattedBlock` struct. Contains recursive children.
  """

  @type t :: %__MODULE__{
          block_type: String.t(),
          level: non_neg_integer() | nil,
          inline_content: list(Kreuzberg.DjotInlineElement.t()),
          attributes: Kreuzberg.DjotAttributes.t() | nil,
          language: String.t() | nil,
          code: String.t() | nil,
          children: list(t())
        }

  defstruct [
    :level,
    :attributes,
    :language,
    :code,
    block_type: "paragraph",
    inline_content: [],
    children: []
  ]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      block_type: data["block_type"] || "paragraph",
      level: data["level"],
      inline_content:
        normalize_list(data["inline_content"], &Kreuzberg.DjotInlineElement.from_map/1),
      attributes: maybe_attrs(data["attributes"]),
      language: data["language"],
      code: data["code"],
      children: normalize_list(data["children"], &from_map/1)
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = block) do
    %{
      "block_type" => block.block_type,
      "level" => block.level,
      "inline_content" => Enum.map(block.inline_content, &Kreuzberg.DjotInlineElement.to_map/1),
      "attributes" =>
        case block.attributes do
          nil -> nil
          a -> Kreuzberg.DjotAttributes.to_map(a)
        end,
      "language" => block.language,
      "code" => block.code,
      "children" => Enum.map(block.children, &to_map/1)
    }
  end

  defp maybe_attrs(nil), do: nil
  defp maybe_attrs(%Kreuzberg.DjotAttributes{} = a), do: a
  defp maybe_attrs(map) when is_map(map), do: Kreuzberg.DjotAttributes.from_map(map)

  defp normalize_list(nil, _fun), do: []
  defp normalize_list(list, fun) when is_list(list), do: Enum.map(list, fun)
end

defmodule Kreuzberg.DjotImage do
  @moduledoc """
  Image element in a Djot document.

  Matches the Rust `DjotImage` struct.
  """

  @type t :: %__MODULE__{
          src: String.t(),
          alt: String.t(),
          title: String.t() | nil,
          attributes: Kreuzberg.DjotAttributes.t() | nil
        }

  defstruct [:title, :attributes, src: "", alt: ""]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      src: data["src"] || "",
      alt: data["alt"] || "",
      title: data["title"],
      attributes: maybe_attrs(data["attributes"])
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = img) do
    %{
      "src" => img.src,
      "alt" => img.alt,
      "title" => img.title,
      "attributes" =>
        case img.attributes do
          nil -> nil
          a -> Kreuzberg.DjotAttributes.to_map(a)
        end
    }
  end

  defp maybe_attrs(nil), do: nil
  defp maybe_attrs(%Kreuzberg.DjotAttributes{} = a), do: a
  defp maybe_attrs(map) when is_map(map), do: Kreuzberg.DjotAttributes.from_map(map)
end

defmodule Kreuzberg.DjotLink do
  @moduledoc """
  Link element in a Djot document.

  Matches the Rust `DjotLink` struct.
  """

  @type t :: %__MODULE__{
          url: String.t(),
          text: String.t(),
          title: String.t() | nil,
          attributes: Kreuzberg.DjotAttributes.t() | nil
        }

  defstruct [:title, :attributes, url: "", text: ""]

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      url: data["url"] || "",
      text: data["text"] || "",
      title: data["title"],
      attributes: maybe_attrs(data["attributes"])
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = link) do
    %{
      "url" => link.url,
      "text" => link.text,
      "title" => link.title,
      "attributes" =>
        case link.attributes do
          nil -> nil
          a -> Kreuzberg.DjotAttributes.to_map(a)
        end
    }
  end

  defp maybe_attrs(nil), do: nil
  defp maybe_attrs(%Kreuzberg.DjotAttributes{} = a), do: a
  defp maybe_attrs(map) when is_map(map), do: Kreuzberg.DjotAttributes.from_map(map)
end

defmodule Kreuzberg.DjotFootnote do
  @moduledoc """
  Footnote in a Djot document.

  Matches the Rust `Footnote` struct.
  """

  @type t :: %__MODULE__{
          label: String.t(),
          content: list(Kreuzberg.DjotFormattedBlock.t())
        }

  defstruct label: "", content: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      label: data["label"] || "",
      content: normalize_list(data["content"], &Kreuzberg.DjotFormattedBlock.from_map/1)
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = fn_) do
    %{
      "label" => fn_.label,
      "content" => Enum.map(fn_.content, &Kreuzberg.DjotFormattedBlock.to_map/1)
    }
  end

  defp normalize_list(nil, _fun), do: []
  defp normalize_list(list, fun) when is_list(list), do: Enum.map(list, fun)
end

defmodule Kreuzberg.DjotContent do
  @moduledoc """
  Comprehensive Djot document structure with semantic preservation.

  Matches the Rust `DjotContent` struct.

  ## Fields

    * `:plain_text` - Plain text representation for backwards compatibility
    * `:blocks` - Structured block-level content
    * `:metadata` - Metadata from YAML frontmatter
    * `:tables` - Extracted tables as structured data
    * `:images` - Extracted images with metadata
    * `:links` - Extracted links with URLs
    * `:footnotes` - Footnote definitions
    * `:attributes` - Attributes mapped by element identifier
  """

  @type t :: %__MODULE__{
          plain_text: String.t(),
          blocks: list(Kreuzberg.DjotFormattedBlock.t()),
          metadata: Kreuzberg.Metadata.t(),
          tables: list(Kreuzberg.Table.t()),
          images: list(Kreuzberg.DjotImage.t()),
          links: list(Kreuzberg.DjotLink.t()),
          footnotes: list(Kreuzberg.DjotFootnote.t()),
          attributes: list({String.t(), Kreuzberg.DjotAttributes.t()})
        }

  defstruct plain_text: "",
            blocks: [],
            metadata: %Kreuzberg.Metadata{},
            tables: [],
            images: [],
            links: [],
            footnotes: [],
            attributes: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      plain_text: data["plain_text"] || "",
      blocks: normalize_list(data["blocks"], &Kreuzberg.DjotFormattedBlock.from_map/1),
      metadata: normalize_metadata(data["metadata"]),
      tables: normalize_list(data["tables"], &Kreuzberg.Table.from_map/1),
      images: normalize_list(data["images"], &Kreuzberg.DjotImage.from_map/1),
      links: normalize_list(data["links"], &Kreuzberg.DjotLink.from_map/1),
      footnotes: normalize_list(data["footnotes"], &Kreuzberg.DjotFootnote.from_map/1),
      attributes: normalize_attributes(data["attributes"])
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = djot) do
    %{
      "plain_text" => djot.plain_text,
      "blocks" => Enum.map(djot.blocks, &Kreuzberg.DjotFormattedBlock.to_map/1),
      "metadata" => Kreuzberg.Metadata.to_map(djot.metadata),
      "tables" => Enum.map(djot.tables, &Kreuzberg.Table.to_map/1),
      "images" => Enum.map(djot.images, &Kreuzberg.DjotImage.to_map/1),
      "links" => Enum.map(djot.links, &Kreuzberg.DjotLink.to_map/1),
      "footnotes" => Enum.map(djot.footnotes, &Kreuzberg.DjotFootnote.to_map/1),
      "attributes" =>
        Enum.map(djot.attributes, fn {k, a} -> [k, Kreuzberg.DjotAttributes.to_map(a)] end)
    }
  end

  defp normalize_metadata(nil), do: %Kreuzberg.Metadata{}
  defp normalize_metadata(%Kreuzberg.Metadata{} = m), do: m
  defp normalize_metadata(map) when is_map(map), do: Kreuzberg.Metadata.from_map(map)

  defp normalize_list(nil, _fun), do: []
  defp normalize_list(list, fun) when is_list(list), do: Enum.map(list, fun)

  # serde serializes Vec<(String, Attributes)> as [[key, attrs_map], ...]
  defp normalize_attributes(nil), do: []

  defp normalize_attributes(list) when is_list(list) do
    Enum.map(list, fn
      [k, v] when is_map(v) -> {k, Kreuzberg.DjotAttributes.from_map(v)}
      {k, %Kreuzberg.DjotAttributes{} = a} -> {k, a}
      {k, v} when is_map(v) -> {k, Kreuzberg.DjotAttributes.from_map(v)}
      other -> other
    end)
  end
end

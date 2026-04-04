defmodule Kreuzberg.DocumentTextAnnotation do
  @moduledoc """
  Inline text annotation with byte-range formatting and links.

  Annotations reference byte offsets into a node's text content,
  enabling precise identification of formatted regions.

  ## Fields

    * `:start` - Start byte offset (inclusive)
    * `:end` - End byte offset (exclusive)
    * `:kind` - Annotation type (bold, italic, link, etc.)
    * `:url` - URL for link annotations (nil for other types)

  ## Examples

      iex> annotation = %Kreuzberg.DocumentTextAnnotation{
      ...>   start: 0,
      ...>   end: 5,
      ...>   kind: "bold"
      ...> }
      iex> annotation.kind
      "bold"
  """

  @type annotation_kind ::
          :bold
          | :italic
          | :underline
          | :strikethrough
          | :code
          | :subscript
          | :superscript
          | :link

  @type t :: %__MODULE__{
          start: non_neg_integer(),
          end: non_neg_integer(),
          kind: String.t(),
          url: String.t() | nil
        }

  defstruct [:start, :end, :kind, :url]

  @doc """
  Creates a DocumentTextAnnotation struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct.

  ## Parameters

    * `data` - A map containing annotation fields

  ## Returns

  A `DocumentTextAnnotation` struct with properly typed fields.

  ## Examples

      iex> annotation_map = %{
      ...>   "start" => 0,
      ...>   "end" => 5,
      ...>   "kind" => "bold"
      ...> }
      iex> annotation = Kreuzberg.DocumentTextAnnotation.from_map(annotation_map)
      iex> annotation.kind
      "bold"
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      start: data["start"] || 0,
      end: data["end"] || 0,
      kind: data["kind"] || data["annotation_type"] || "",
      url: data["url"]
    }
  end

  @doc """
  Converts a DocumentTextAnnotation struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `annotation` - A `DocumentTextAnnotation` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> annotation = %Kreuzberg.DocumentTextAnnotation{
      ...>   start: 0,
      ...>   end: 5,
      ...>   kind: "bold"
      ...> }
      iex> Kreuzberg.DocumentTextAnnotation.to_map(annotation)
      %{"start" => 0, "end" => 5, "kind" => "bold", "url" => nil}
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = annotation) do
    %{
      "start" => annotation.start,
      "end" => annotation.end,
      "kind" => annotation.kind,
      "url" => annotation.url
    }
  end

  @doc """
  Get human-readable annotation kind name.

  Converts annotation kind to a display-friendly string.

  ## Parameters

    * `annotation` - A `DocumentTextAnnotation` struct

  ## Returns

  A human-readable string representation of the annotation kind.

  ## Examples

      iex> annotation = %Kreuzberg.DocumentTextAnnotation{kind: "bold"}
      iex> Kreuzberg.DocumentTextAnnotation.readable_kind(annotation)
      "Bold"
  """
  @spec readable_kind(t()) :: String.t()
  def readable_kind(%__MODULE__{kind: kind}) do
    case kind do
      "bold" -> "Bold"
      "italic" -> "Italic"
      "underline" -> "Underline"
      "strikethrough" -> "Strikethrough"
      "code" -> "Code"
      "subscript" -> "Subscript"
      "superscript" -> "Superscript"
      "link" -> "Link"
      _ -> String.capitalize(kind)
    end
  end

  @doc """
  Check if this is a link annotation.

  ## Parameters

    * `annotation` - A `DocumentTextAnnotation` struct

  ## Returns

  Boolean indicating whether the annotation is a link.

  ## Examples

      iex> annotation = %Kreuzberg.DocumentTextAnnotation{kind: "link", url: "https://example.com"}
      iex> Kreuzberg.DocumentTextAnnotation.link?(annotation)
      true
  """
  @spec link?(t()) :: boolean()
  def link?(%__MODULE__{kind: "link"}), do: true
  def link?(%__MODULE__{}), do: false
end

defmodule Kreuzberg.DocumentNode do
  @moduledoc """
  A single node in the document tree.

  Each node has a deterministic ID, typed content, optional parent/children references,
  and metadata like page number and content layer classification.

  ## Fields

    * `:id` - Deterministic identifier (hash of content + position)
    * `:node_type` - Node type discriminant (paragraph, heading, list, etc.)
    * `:content` - Node content as a map with type-specific fields
    * `:content_layer` - Content layer classification (body, header, footer, footnote)
    * `:parent` - Parent node index (nil if root node)
    * `:children` - List of child node indices in reading order
    * `:page_number` - Page number where node starts (1-indexed)
    * `:page_number_end` - Page number where node ends (for multi-page elements)
    * `:bbox` - Bounding box coordinates if available
    * `:annotations` - List of inline text annotations

  ## Examples

      iex> node = %Kreuzberg.DocumentNode{
      ...>   id: "node-1",
      ...>   node_type: "paragraph",
      ...>   content: %{"text" => "Hello world"},
      ...>   page_number: 1
      ...> }
      iex> node.node_type
      "paragraph"
  """

  @type node_type ::
          :title
          | :heading
          | :paragraph
          | :list
          | :list_item
          | :table
          | :image
          | :code
          | :quote
          | :formula
          | :footnote
          | :group
          | :page_break

  @type t :: %__MODULE__{
          id: String.t(),
          node_type: String.t(),
          content: map(),
          content_layer: String.t() | nil,
          parent: non_neg_integer() | nil,
          children: list(non_neg_integer()),
          page_number: non_neg_integer() | nil,
          page_number_end: non_neg_integer() | nil,
          bbox: Kreuzberg.BoundingBox.t() | nil,
          annotations: list(Kreuzberg.DocumentTextAnnotation.t())
        }

  defstruct [
    :id,
    :node_type,
    :content,
    :content_layer,
    :parent,
    :page_number,
    :page_number_end,
    :bbox,
    children: [],
    annotations: []
  ]

  @doc """
  Creates a DocumentNode struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct,
  handling nested content and annotation data.

  ## Parameters

    * `data` - A map containing node fields

  ## Returns

  A `DocumentNode` struct with properly typed fields.

  ## Examples

      iex> node_map = %{
      ...>   "id" => "node-1",
      ...>   "node_type" => "paragraph",
      ...>   "content" => %{"text" => "Hello"},
      ...>   "page" => 1
      ...> }
      iex> node = Kreuzberg.DocumentNode.from_map(node_map)
      iex> node.node_type
      "paragraph"
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    children =
      case data["children"] do
        nil -> []
        list when is_list(list) -> Enum.filter(list, &is_integer/1)
        _ -> []
      end

    bbox =
      case data["bbox"] do
        nil -> nil
        %Kreuzberg.BoundingBox{} = b -> b
        map when is_map(map) -> Kreuzberg.BoundingBox.from_map(map)
        _ -> nil
      end

    annotations =
      case data["annotations"] do
        nil ->
          []

        list when is_list(list) ->
          Enum.map(list, fn
            %Kreuzberg.DocumentTextAnnotation{} = ann -> ann
            map when is_map(map) -> Kreuzberg.DocumentTextAnnotation.from_map(map)
            _ -> nil
          end)
          |> Enum.reject(&is_nil/1)

        _ ->
          []
      end

    content_map = data["content"] || %{}

    %__MODULE__{
      id: data["id"] || "",
      node_type: (is_map(content_map) && content_map["node_type"]) || "",
      content: content_map,
      content_layer: data["content_layer"],
      parent: data["parent"],
      children: children,
      page_number: data["page"],
      page_number_end: data["page_end"],
      bbox: bbox,
      annotations: annotations
    }
  end

  @doc """
  Converts a DocumentNode struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `node` - A `DocumentNode` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> node = %Kreuzberg.DocumentNode{
      ...>   id: "node-1",
      ...>   node_type: "paragraph",
      ...>   content: %{"text" => "Hello"}
      ...> }
      iex> map = Kreuzberg.DocumentNode.to_map(node)
      iex> map["node_type"]
      "paragraph"
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = node) do
    %{
      "id" => node.id,
      "node_type" => node.node_type,
      "content" => node.content,
      "content_layer" => node.content_layer,
      "parent" => node.parent,
      "children" => node.children,
      "page" => node.page_number,
      "page_end" => node.page_number_end,
      "bbox" =>
        case node.bbox do
          nil -> nil
          bbox -> Kreuzberg.BoundingBox.to_map(bbox)
        end,
      "annotations" => Enum.map(node.annotations, &Kreuzberg.DocumentTextAnnotation.to_map/1)
    }
  end

  @doc """
  Check if this is a root node (no parent).

  ## Parameters

    * `node` - A `DocumentNode` struct

  ## Returns

  Boolean indicating whether the node is a root node.

  ## Examples

      iex> node = %Kreuzberg.DocumentNode{parent: nil}
      iex> Kreuzberg.DocumentNode.root?(node)
      true
  """
  @spec root?(t()) :: boolean()
  def root?(%__MODULE__{parent: nil}), do: true
  def root?(%__MODULE__{}), do: false

  @doc """
  Check if this node has children.

  ## Parameters

    * `node` - A `DocumentNode` struct

  ## Returns

  Boolean indicating whether the node has child nodes.

  ## Examples

      iex> node = %Kreuzberg.DocumentNode{children: [1, 2]}
      iex> Kreuzberg.DocumentNode.has_children?(node)
      true
  """
  @spec has_children?(t()) :: boolean()
  def has_children?(%__MODULE__{children: children}), do: children != []

  @doc """
  Get node type with readable formatting.

  Converts snake_case node types to Title Case for display.

  ## Parameters

    * `node` - A `DocumentNode` struct

  ## Returns

  A human-readable string representation of the node type.

  ## Examples

      iex> node = %Kreuzberg.DocumentNode{node_type: "list_item"}
      iex> Kreuzberg.DocumentNode.readable_type(node)
      "List Item"
  """
  @spec readable_type(t()) :: String.t()
  def readable_type(%__MODULE__{node_type: type}) do
    case type do
      "title" -> "Title"
      "heading" -> "Heading"
      "paragraph" -> "Paragraph"
      "list" -> "List"
      "list_item" -> "List Item"
      "table" -> "Table"
      "image" -> "Image"
      "code" -> "Code"
      "quote" -> "Quote"
      "formula" -> "Formula"
      "footnote" -> "Footnote"
      "group" -> "Group"
      "page_break" -> "Page Break"
      _ -> String.replace(type, "_", " ") |> String.capitalize()
    end
  end
end

defmodule Kreuzberg.DocumentStructure do
  @moduledoc """
  Structured document representation with hierarchical node tree.

  A flat array of nodes with index-based parent/child references forming a tree.
  Root-level nodes have no parent. Nodes are stored in document/reading order.

  ## Fields

    * `:nodes` - List of DocumentNode structs in reading order

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{
      ...>   nodes: [
      ...>     %Kreuzberg.DocumentNode{
      ...>       id: "node-1",
      ...>       node_type: "paragraph",
      ...>       content: %{"text" => "Hello world"},
      ...>       page_number: 1
      ...>     }
      ...>   ]
      ...> }
      iex> structure.nodes
      [%Kreuzberg.DocumentNode{...}]
  """

  @type t :: %__MODULE__{
          nodes: list(Kreuzberg.DocumentNode.t())
        }

  defstruct [:nodes]

  @doc """
  Creates a DocumentStructure struct from a map.

  Converts a plain map (typically from NIF/Rust) into a proper struct,
  handling nested node data.

  ## Parameters

    * `data` - A map containing document structure fields

  ## Returns

  A `DocumentStructure` struct with properly typed fields.

  ## Examples

      iex> structure_map = %{
      ...>   "nodes" => [
      ...>     %{
      ...>       "id" => "node-1",
      ...>       "node_type" => "paragraph",
      ...>       "content" => %{"text" => "Hello"}
      ...>     }
      ...>   ]
      ...> }
      iex> structure = Kreuzberg.DocumentStructure.from_map(structure_map)
      iex> length(structure.nodes)
      1
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    nodes =
      case data["nodes"] do
        nil ->
          []

        nodes_list when is_list(nodes_list) ->
          Enum.map(nodes_list, fn
            %Kreuzberg.DocumentNode{} = node -> node
            map when is_map(map) -> Kreuzberg.DocumentNode.from_map(map)
            _ -> nil
          end)
          |> Enum.reject(&is_nil/1)

        _ ->
          []
      end

    %__MODULE__{
      nodes: nodes
    }
  end

  @doc """
  Converts a DocumentStructure struct to a map.

  Useful for serialization and passing to external systems.

  ## Parameters

    * `structure` - A `DocumentStructure` struct

  ## Returns

  A map with string keys representing all fields.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{nodes: []}
      iex> Kreuzberg.DocumentStructure.to_map(structure)
      %{"nodes" => []}
  """
  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = structure) do
    %{
      "nodes" => Enum.map(structure.nodes, &Kreuzberg.DocumentNode.to_map/1)
    }
  end

  @doc """
  Get total number of nodes in structure.

  ## Parameters

    * `structure` - A `DocumentStructure` struct

  ## Returns

  The count of nodes as an integer.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{nodes: [%Kreuzberg.DocumentNode{}]}
      iex> Kreuzberg.DocumentStructure.count(structure)
      1
  """
  @spec count(t()) :: non_neg_integer()
  def count(%__MODULE__{nodes: nodes}), do: length(nodes)

  @doc """
  Check if document structure is empty.

  ## Parameters

    * `structure` - A `DocumentStructure` struct

  ## Returns

  Boolean indicating whether the structure has no nodes.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{nodes: []}
      iex> Kreuzberg.DocumentStructure.empty?(structure)
      true
  """
  @spec empty?(t()) :: boolean()
  def empty?(%__MODULE__{nodes: nodes}), do: nodes == []

  @doc """
  Get a node by index (0-based).

  ## Parameters

    * `structure` - A `DocumentStructure` struct
    * `index` - Zero-based index of the node to retrieve

  ## Returns

  The node at that index, or nil if out of bounds.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{
      ...>   nodes: [%Kreuzberg.DocumentNode{id: "node-1"}]
      ...> }
      iex> Kreuzberg.DocumentStructure.get_node(structure, 0)
      %Kreuzberg.DocumentNode{id: "node-1", ...}
  """
  @spec get_node(t(), non_neg_integer()) :: Kreuzberg.DocumentNode.t() | nil
  def get_node(%__MODULE__{nodes: nodes}, index) when is_integer(index) and index >= 0 do
    Enum.at(nodes, index)
  end

  def get_node(_, _), do: nil

  @doc """
  Get all root-level nodes (nodes with no parent).

  ## Parameters

    * `structure` - A `DocumentStructure` struct

  ## Returns

  A list of root-level nodes.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{
      ...>   nodes: [
      ...>     %Kreuzberg.DocumentNode{id: "node-1", parent: nil},
      ...>     %Kreuzberg.DocumentNode{id: "node-2", parent: 0}
      ...>   ]
      ...> }
      iex> roots = Kreuzberg.DocumentStructure.root_nodes(structure)
      iex> length(roots)
      1
  """
  @spec root_nodes(t()) :: list(Kreuzberg.DocumentNode.t())
  def root_nodes(%__MODULE__{nodes: nodes}) do
    Enum.filter(nodes, fn node -> is_nil(node.parent) end)
  end

  @doc """
  Get all nodes of a specific type.

  ## Parameters

    * `structure` - A `DocumentStructure` struct
    * `node_type` - The node type to filter by (string or atom)

  ## Returns

  A list of nodes matching the given type.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{
      ...>   nodes: [
      ...>     %Kreuzberg.DocumentNode{node_type: "paragraph"},
      ...>     %Kreuzberg.DocumentNode{node_type: "heading"},
      ...>     %Kreuzberg.DocumentNode{node_type: "paragraph"}
      ...>   ]
      ...> }
      iex> paragraphs = Kreuzberg.DocumentStructure.nodes_by_type(structure, "paragraph")
      iex> length(paragraphs)
      2
  """
  @spec nodes_by_type(t(), String.t() | atom()) :: list(Kreuzberg.DocumentNode.t())
  def nodes_by_type(%__MODULE__{nodes: nodes}, node_type) when is_binary(node_type) do
    Enum.filter(nodes, fn node -> node.node_type === node_type end)
  end

  def nodes_by_type(%__MODULE__{nodes: nodes}, node_type) when is_atom(node_type) do
    type_str = Atom.to_string(node_type)
    Enum.filter(nodes, fn node -> node.node_type === type_str end)
  end

  @doc """
  Get child nodes of a specific parent node.

  ## Parameters

    * `structure` - A `DocumentStructure` struct
    * `parent_index` - The index of the parent node

  ## Returns

  A list of child nodes.

  ## Examples

      iex> structure = %Kreuzberg.DocumentStructure{
      ...>   nodes: [
      ...>     %Kreuzberg.DocumentNode{id: "node-1", children: [1, 2]},
      ...>     %Kreuzberg.DocumentNode{id: "node-2", parent: 0},
      ...>     %Kreuzberg.DocumentNode{id: "node-3", parent: 0}
      ...>   ]
      ...> }
      iex> children = Kreuzberg.DocumentStructure.children(structure, 0)
      iex> length(children)
      2
  """
  @spec children(t(), non_neg_integer()) :: list(Kreuzberg.DocumentNode.t())
  def children(%__MODULE__{nodes: nodes}, parent_index) when is_integer(parent_index) do
    case Enum.at(nodes, parent_index) do
      nil ->
        []

      parent ->
        Enum.map(parent.children, &Enum.at(nodes, &1))
        |> Enum.reject(&is_nil/1)
    end
  end

  def children(_, _), do: []
end

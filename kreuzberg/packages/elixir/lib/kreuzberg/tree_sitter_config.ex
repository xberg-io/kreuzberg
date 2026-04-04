defmodule Kreuzberg.TreeSitterProcessConfig do
  @moduledoc """
  Tree-sitter process configuration for code extraction.

  Controls which code elements are extracted during tree-sitter parsing.

  ## Fields

    * `:structure` - Extract structural items (default: true)
    * `:imports` - Extract import statements (default: true)
    * `:exports` - Extract export statements (default: true)
    * `:comments` - Extract comments (default: false)
    * `:docstrings` - Extract docstrings (default: false)
    * `:symbols` - Extract symbol definitions (default: false)
    * `:diagnostics` - Include parse diagnostics (default: false)
    * `:chunk_max_size` - Maximum chunk size in bytes, nil disables chunking
  """

  @type t :: %__MODULE__{
          structure: boolean(),
          imports: boolean(),
          exports: boolean(),
          comments: boolean(),
          docstrings: boolean(),
          symbols: boolean(),
          diagnostics: boolean(),
          chunk_max_size: non_neg_integer() | nil,
          content_mode: String.t() | nil
        }

  @derive Jason.Encoder
  defstruct [
    :chunk_max_size,
    :content_mode,
    structure: true,
    imports: true,
    exports: true,
    comments: false,
    docstrings: false,
    symbols: false,
    diagnostics: false
  ]

  @doc """
  Creates a TreeSitterProcessConfig struct from a map.

  ## Examples

      iex> Kreuzberg.TreeSitterProcessConfig.from_map(%{"structure" => true, "comments" => true})
      %Kreuzberg.TreeSitterProcessConfig{structure: true, comments: true}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    %__MODULE__{
      structure: Map.get(data, "structure", Map.get(data, :structure, true)),
      imports: Map.get(data, "imports", Map.get(data, :imports, true)),
      exports: Map.get(data, "exports", Map.get(data, :exports, true)),
      comments: Map.get(data, "comments", Map.get(data, :comments, false)),
      docstrings: Map.get(data, "docstrings", Map.get(data, :docstrings, false)),
      symbols: Map.get(data, "symbols", Map.get(data, :symbols, false)),
      diagnostics: Map.get(data, "diagnostics", Map.get(data, :diagnostics, false)),
      chunk_max_size: Map.get(data, "chunk_max_size", Map.get(data, :chunk_max_size)),
      content_mode: Map.get(data, "content_mode", Map.get(data, :content_mode))
    }
  end

  @doc """
  Converts a TreeSitterProcessConfig struct to a map.
  """
  @spec to_map(t()) :: map()
  def to_map(map) when is_map(map) and not is_struct(map), do: map

  def to_map(%__MODULE__{} = config) do
    %{
      "structure" => config.structure,
      "imports" => config.imports,
      "exports" => config.exports,
      "comments" => config.comments,
      "docstrings" => config.docstrings,
      "symbols" => config.symbols,
      "diagnostics" => config.diagnostics,
      "chunk_max_size" => config.chunk_max_size,
      "content_mode" => config.content_mode
    }
  end
end

defmodule Kreuzberg.TreeSitterConfig do
  @moduledoc """
  Tree-sitter configuration for code parsing.

  Configures tree-sitter grammar management and code extraction behavior.

  ## Fields

    * `:cache_dir` - Custom cache directory for downloaded grammars
    * `:languages` - Languages to pre-download on init
    * `:groups` - Language groups to pre-download
    * `:process` - Tree-sitter process configuration
  """

  alias Kreuzberg.TreeSitterProcessConfig

  @type t :: %__MODULE__{
          enabled: boolean() | nil,
          cache_dir: String.t() | nil,
          languages: [String.t()] | nil,
          groups: [String.t()] | nil,
          process: TreeSitterProcessConfig.t() | nil
        }

  @derive Jason.Encoder
  defstruct [
    :enabled,
    :cache_dir,
    :languages,
    :groups,
    :process
  ]

  @doc """
  Creates a TreeSitterConfig struct from a map.

  ## Examples

      iex> Kreuzberg.TreeSitterConfig.from_map(%{"cache_dir" => "/tmp/grammars"})
      %Kreuzberg.TreeSitterConfig{cache_dir: "/tmp/grammars"}
  """
  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    process =
      case Map.get(data, "process", Map.get(data, :process)) do
        %TreeSitterProcessConfig{} = p -> p
        %{} = p -> TreeSitterProcessConfig.from_map(p)
        _ -> nil
      end

    %__MODULE__{
      enabled: Map.get(data, "enabled", Map.get(data, :enabled)),
      cache_dir: Map.get(data, "cache_dir", Map.get(data, :cache_dir)),
      languages: Map.get(data, "languages", Map.get(data, :languages)),
      groups: Map.get(data, "groups", Map.get(data, :groups)),
      process: process
    }
  end

  @doc """
  Converts a TreeSitterConfig struct to a map.
  """
  @spec to_map(t()) :: map()
  def to_map(map) when is_map(map) and not is_struct(map), do: map

  def to_map(%__MODULE__{} = config) do
    %{
      "enabled" => config.enabled,
      "cache_dir" => config.cache_dir,
      "languages" => config.languages,
      "groups" => config.groups,
      "process" =>
        if config.process do
          TreeSitterProcessConfig.to_map(config.process)
        end
    }
  end
end

defmodule Kreuzberg.CodeProcessResult do
  @moduledoc """
  Result of tree-sitter code processing.

  ## Fields

    * `:language` - Detected programming language
    * `:metrics` - File-level code metrics
    * `:structure` - Structural items (functions, classes, etc.)
    * `:imports` - Import statements
    * `:exports` - Export statements
    * `:comments` - Comments
    * `:docstrings` - Docstrings
    * `:symbols` - Symbol definitions
    * `:diagnostics` - Parse diagnostics
    * `:chunks` - Code chunks
  """

  @type t :: %__MODULE__{
          language: String.t(),
          metrics: Kreuzberg.CodeFileMetrics.t(),
          structure: [Kreuzberg.CodeStructureItem.t()],
          imports: [Kreuzberg.CodeImportInfo.t()],
          exports: [Kreuzberg.CodeExportInfo.t()],
          comments: [Kreuzberg.CodeCommentInfo.t()],
          docstrings: [Kreuzberg.CodeDocstringInfo.t()],
          symbols: [Kreuzberg.CodeSymbolInfo.t()],
          diagnostics: [Kreuzberg.CodeDiagnostic.t()],
          chunks: [Kreuzberg.CodeChunk.t()]
        }

  defstruct language: "",
            metrics: %Kreuzberg.CodeFileMetrics{},
            structure: [],
            imports: [],
            exports: [],
            comments: [],
            docstrings: [],
            symbols: [],
            diagnostics: [],
            chunks: []

  @spec from_map(map()) :: t()
  def from_map(data) when is_map(data) do
    metrics =
      case Map.get(data, "metrics", Map.get(data, :metrics)) do
        %Kreuzberg.CodeFileMetrics{} = m -> m
        map when is_map(map) -> Kreuzberg.CodeFileMetrics.from_map(map)
        _ -> %Kreuzberg.CodeFileMetrics{}
      end

    %__MODULE__{
      language: Map.get(data, "language", Map.get(data, :language, "")),
      metrics: metrics,
      structure: map_list(data, "structure", :structure, &Kreuzberg.CodeStructureItem.from_map/1),
      imports: map_list(data, "imports", :imports, &Kreuzberg.CodeImportInfo.from_map/1),
      exports: map_list(data, "exports", :exports, &Kreuzberg.CodeExportInfo.from_map/1),
      comments: map_list(data, "comments", :comments, &Kreuzberg.CodeCommentInfo.from_map/1),
      docstrings:
        map_list(data, "docstrings", :docstrings, &Kreuzberg.CodeDocstringInfo.from_map/1),
      symbols: map_list(data, "symbols", :symbols, &Kreuzberg.CodeSymbolInfo.from_map/1),
      diagnostics:
        map_list(data, "diagnostics", :diagnostics, &Kreuzberg.CodeDiagnostic.from_map/1),
      chunks: map_list(data, "chunks", :chunks, &Kreuzberg.CodeChunk.from_map/1)
    }
  end

  @spec to_map(t()) :: map()
  def to_map(%__MODULE__{} = result) do
    %{
      "language" => result.language,
      "metrics" => Kreuzberg.CodeFileMetrics.to_map(result.metrics),
      "structure" => Enum.map(result.structure, &Kreuzberg.CodeStructureItem.to_map/1),
      "imports" => Enum.map(result.imports, &Kreuzberg.CodeImportInfo.to_map/1),
      "exports" => Enum.map(result.exports, &Kreuzberg.CodeExportInfo.to_map/1),
      "comments" => Enum.map(result.comments, &Kreuzberg.CodeCommentInfo.to_map/1),
      "docstrings" => Enum.map(result.docstrings, &Kreuzberg.CodeDocstringInfo.to_map/1),
      "symbols" => Enum.map(result.symbols, &Kreuzberg.CodeSymbolInfo.to_map/1),
      "diagnostics" => Enum.map(result.diagnostics, &Kreuzberg.CodeDiagnostic.to_map/1),
      "chunks" => Enum.map(result.chunks, &Kreuzberg.CodeChunk.to_map/1)
    }
  end

  defp map_list(data, string_key, atom_key, mapper) do
    case Map.get(data, string_key, Map.get(data, atom_key, [])) do
      list when is_list(list) -> Enum.map(list, mapper)
      _ -> []
    end
  end
end

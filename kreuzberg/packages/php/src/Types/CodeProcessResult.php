<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Result of tree-sitter code processing.
 *
 * @property-read string $language Detected programming language
 * @property-read CodeFileMetrics $metrics File-level code metrics
 * @property-read array<CodeStructureItem> $structure Structural items (functions, classes, etc.)
 * @property-read array<CodeImportInfo> $imports Import statements
 * @property-read array<CodeExportInfo> $exports Export statements
 * @property-read array<CodeCommentInfo> $comments Comments
 * @property-read array<CodeDocstringInfo> $docstrings Docstrings
 * @property-read array<CodeSymbolInfo> $symbols Symbol definitions
 * @property-read array<CodeDiagnostic> $diagnostics Parse diagnostics
 * @property-read array<CodeChunk> $chunks Code chunks
 */
readonly class CodeProcessResult
{
    /**
     * @param array<CodeStructureItem> $structure
     * @param array<CodeImportInfo> $imports
     * @param array<CodeExportInfo> $exports
     * @param array<CodeCommentInfo> $comments
     * @param array<CodeDocstringInfo> $docstrings
     * @param array<CodeSymbolInfo> $symbols
     * @param array<CodeDiagnostic> $diagnostics
     * @param array<CodeChunk> $chunks
     */
    public function __construct(
        public string $language,
        public CodeFileMetrics $metrics,
        public array $structure = [],
        public array $imports = [],
        public array $exports = [],
        public array $comments = [],
        public array $docstrings = [],
        public array $symbols = [],
        public array $diagnostics = [],
        public array $chunks = [],
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $language */
        $language = $data['language'] ?? '';

        /** @var array<string, mixed> $metricsData */
        $metricsData = $data['metrics'] ?? [];

        /** @var array<array<string, mixed>> $structureData */
        $structureData = $data['structure'] ?? [];

        /** @var array<array<string, mixed>> $importsData */
        $importsData = $data['imports'] ?? [];

        /** @var array<array<string, mixed>> $exportsData */
        $exportsData = $data['exports'] ?? [];

        /** @var array<array<string, mixed>> $commentsData */
        $commentsData = $data['comments'] ?? [];

        /** @var array<array<string, mixed>> $docstringsData */
        $docstringsData = $data['docstrings'] ?? [];

        /** @var array<array<string, mixed>> $symbolsData */
        $symbolsData = $data['symbols'] ?? [];

        /** @var array<array<string, mixed>> $diagnosticsData */
        $diagnosticsData = $data['diagnostics'] ?? [];

        /** @var array<array<string, mixed>> $chunksData */
        $chunksData = $data['chunks'] ?? [];

        return new self(
            language: $language,
            metrics: CodeFileMetrics::fromArray($metricsData),
            structure: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeStructureItem => CodeStructureItem::fromArray($item),
                $structureData,
            ),
            imports: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeImportInfo => CodeImportInfo::fromArray($item),
                $importsData,
            ),
            exports: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeExportInfo => CodeExportInfo::fromArray($item),
                $exportsData,
            ),
            comments: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeCommentInfo => CodeCommentInfo::fromArray($item),
                $commentsData,
            ),
            docstrings: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeDocstringInfo => CodeDocstringInfo::fromArray($item),
                $docstringsData,
            ),
            symbols: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeSymbolInfo => CodeSymbolInfo::fromArray($item),
                $symbolsData,
            ),
            diagnostics: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeDiagnostic => CodeDiagnostic::fromArray($item),
                $diagnosticsData,
            ),
            chunks: array_map(
                /** @param array<string, mixed> $item */
                static fn (array $item): CodeChunk => CodeChunk::fromArray($item),
                $chunksData,
            ),
        );
    }
}

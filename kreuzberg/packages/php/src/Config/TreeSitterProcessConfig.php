<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Tree-sitter process configuration.
 *
 * Controls which code elements are extracted during tree-sitter parsing,
 * including structural items, imports, exports, comments, docstrings,
 * symbols, and diagnostics.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\TreeSitterProcessConfig;
 *
 * $process = new TreeSitterProcessConfig(
 *     structure: true,
 *     imports: true,
 *     exports: true,
 *     comments: true,
 *     symbols: true,
 * );
 * ```
 */
readonly class TreeSitterProcessConfig
{
    /**
     * @param bool $structure Extract structural items. Default true.
     * @param bool $imports Extract import statements. Default true.
     * @param bool $exports Extract export statements. Default true.
     * @param bool $comments Extract comments. Default false.
     * @param bool $docstrings Extract docstrings. Default false.
     * @param bool $symbols Extract symbol definitions. Default false.
     * @param bool $diagnostics Include parse diagnostics. Default false.
     * @param int|null $chunkMaxSize Maximum chunk size in bytes. Null disables chunking.
     */
    public function __construct(
        public bool $structure = true,
        public bool $imports = true,
        public bool $exports = true,
        public bool $comments = false,
        public bool $docstrings = false,
        public bool $symbols = false,
        public bool $diagnostics = false,
        public ?int $chunkMaxSize = null,
        public ?string $contentMode = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var bool $structure */
        $structure = $data['structure'] ?? true;
        if (!is_bool($structure)) {
            $structure = (bool) $structure;
        }

        /** @var bool $imports */
        $imports = $data['imports'] ?? true;
        if (!is_bool($imports)) {
            $imports = (bool) $imports;
        }

        /** @var bool $exports */
        $exports = $data['exports'] ?? true;
        if (!is_bool($exports)) {
            $exports = (bool) $exports;
        }

        /** @var bool $comments */
        $comments = $data['comments'] ?? false;
        if (!is_bool($comments)) {
            $comments = (bool) $comments;
        }

        /** @var bool $docstrings */
        $docstrings = $data['docstrings'] ?? false;
        if (!is_bool($docstrings)) {
            $docstrings = (bool) $docstrings;
        }

        /** @var bool $symbols */
        $symbols = $data['symbols'] ?? false;
        if (!is_bool($symbols)) {
            $symbols = (bool) $symbols;
        }

        /** @var bool $diagnostics */
        $diagnostics = $data['diagnostics'] ?? false;
        if (!is_bool($diagnostics)) {
            $diagnostics = (bool) $diagnostics;
        }

        $chunkMaxSize = $data['chunk_max_size'] ?? null;

        /** @var string|null $contentMode */
        $contentMode = $data['content_mode'] ?? null;
        if ($contentMode !== null && !is_string($contentMode)) {
            $contentMode = null;
        }

        return new self(
            structure: $structure,
            imports: $imports,
            exports: $exports,
            comments: $comments,
            docstrings: $docstrings,
            symbols: $symbols,
            diagnostics: $diagnostics,
            chunkMaxSize: is_int($chunkMaxSize) ? $chunkMaxSize : null,
            contentMode: $contentMode,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [];

        if (!$this->structure) {
            $result['structure'] = false;
        }
        if (!$this->imports) {
            $result['imports'] = false;
        }
        if (!$this->exports) {
            $result['exports'] = false;
        }
        if ($this->comments) {
            $result['comments'] = true;
        }
        if ($this->docstrings) {
            $result['docstrings'] = true;
        }
        if ($this->symbols) {
            $result['symbols'] = true;
        }
        if ($this->diagnostics) {
            $result['diagnostics'] = true;
        }
        if ($this->chunkMaxSize !== null) {
            $result['chunk_max_size'] = $this->chunkMaxSize;
        }
        if ($this->contentMode !== null) {
            $result['content_mode'] = $this->contentMode;
        }

        return $result;
    }
}

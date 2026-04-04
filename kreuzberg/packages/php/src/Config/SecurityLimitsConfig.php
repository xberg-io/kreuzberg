<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Security limits for archive and document extraction.
 *
 * Controls thresholds to prevent resource exhaustion attacks such as
 * decompression bombs, deeply nested archives, and oversized content.
 * When null, default limits from the Rust core are used.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\SecurityLimitsConfig;
 *
 * $limits = new SecurityLimitsConfig(
 *     maxArchiveSize: 100 * 1024 * 1024, // 100 MB
 *     maxFilesInArchive: 1000,
 *     maxNestingDepth: 5,
 * );
 * ```
 */
readonly class SecurityLimitsConfig
{
    public function __construct(
        /**
         * Maximum allowed archive size in bytes.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxArchiveSize = null,

        /**
         * Maximum allowed compression ratio (uncompressed / compressed).
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxCompressionRatio = null,

        /**
         * Maximum number of files allowed inside an archive.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxFilesInArchive = null,

        /**
         * Maximum nesting depth for recursive archive extraction.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxNestingDepth = null,

        /**
         * Maximum length of a single XML/HTML entity.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxEntityLength = null,

        /**
         * Maximum total content size in bytes after extraction.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxContentSize = null,

        /**
         * Maximum number of processing iterations.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxIterations = null,

        /**
         * Maximum XML document nesting depth.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxXmlDepth = null,

        /**
         * Maximum number of cells in a single table.
         *
         * @var int|null
         * @default null (use Rust default)
         */
        public ?int $maxTableCells = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawMaxArchiveSize = $data['max_archive_size'] ?? null;
        $rawMaxCompressionRatio = $data['max_compression_ratio'] ?? null;
        $rawMaxFilesInArchive = $data['max_files_in_archive'] ?? null;
        $rawMaxNestingDepth = $data['max_nesting_depth'] ?? null;
        $rawMaxEntityLength = $data['max_entity_length'] ?? null;
        $rawMaxContentSize = $data['max_content_size'] ?? null;
        $rawMaxIterations = $data['max_iterations'] ?? null;
        $rawMaxXmlDepth = $data['max_xml_depth'] ?? null;
        $rawMaxTableCells = $data['max_table_cells'] ?? null;

        $maxArchiveSize = is_int($rawMaxArchiveSize) ? $rawMaxArchiveSize : null;
        $maxCompressionRatio = is_int($rawMaxCompressionRatio) ? $rawMaxCompressionRatio : null;
        $maxFilesInArchive = is_int($rawMaxFilesInArchive) ? $rawMaxFilesInArchive : null;
        $maxNestingDepth = is_int($rawMaxNestingDepth) ? $rawMaxNestingDepth : null;
        $maxEntityLength = is_int($rawMaxEntityLength) ? $rawMaxEntityLength : null;
        $maxContentSize = is_int($rawMaxContentSize) ? $rawMaxContentSize : null;
        $maxIterations = is_int($rawMaxIterations) ? $rawMaxIterations : null;
        $maxXmlDepth = is_int($rawMaxXmlDepth) ? $rawMaxXmlDepth : null;
        $maxTableCells = is_int($rawMaxTableCells) ? $rawMaxTableCells : null;

        return new self(
            maxArchiveSize: $maxArchiveSize,
            maxCompressionRatio: $maxCompressionRatio,
            maxFilesInArchive: $maxFilesInArchive,
            maxNestingDepth: $maxNestingDepth,
            maxEntityLength: $maxEntityLength,
            maxContentSize: $maxContentSize,
            maxIterations: $maxIterations,
            maxXmlDepth: $maxXmlDepth,
            maxTableCells: $maxTableCells,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, int>
     */
    public function toArray(): array
    {
        /** @var array<string, int> $result */
        $result = array_filter([
            'max_archive_size' => $this->maxArchiveSize,
            'max_compression_ratio' => $this->maxCompressionRatio,
            'max_files_in_archive' => $this->maxFilesInArchive,
            'max_nesting_depth' => $this->maxNestingDepth,
            'max_entity_length' => $this->maxEntityLength,
            'max_content_size' => $this->maxContentSize,
            'max_iterations' => $this->maxIterations,
            'max_xml_depth' => $this->maxXmlDepth,
            'max_table_cells' => $this->maxTableCells,
        ], static fn ($value): bool => $value !== null);

        return $result;
    }
}

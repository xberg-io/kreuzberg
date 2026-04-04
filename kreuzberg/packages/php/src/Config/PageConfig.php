<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Page extraction configuration.
 */
readonly class PageConfig
{
    public function __construct(
        /**
         * Enable page-level extraction metadata.
         *
         * When enabled, the extraction process tracks and returns information
         * about which page each extracted element came from, enabling
         * page-specific queries and reconstruction of page structure.
         *
         * @var bool
         * @default false
         */
        public bool $extractPages = false,

        /**
         * Insert page boundary markers in extracted text.
         *
         * When enabled, special markers indicating page boundaries are inserted
         * into the extracted text stream. This helps downstream processors
         * understand document structure and page breaks.
         *
         * @var bool
         * @default false
         */
        public bool $insertPageMarkers = false,

        /**
         * Format string for page boundary markers.
         *
         * Template for marker text inserted between pages. The placeholder
         * {page_num} is replaced with the actual page number.
         *
         * Valid placeholders:
         * - {page_num}: The page number (1-indexed)
         *
         * Examples:
         * - '--- Page {page_num} ---': Standard page break marker
         * - 'PAGE {page_num}': Simple page header
         * - '[Page {page_num}]': Bracketed format
         * - '\\n\\n=== Page {page_num} ===\\n\\n': Markdown-style separator
         *
         * @var string
         * @default '--- Page {page_num} ---'
         */
        public string $markerFormat = '--- Page {page_num} ---',
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var bool $extractPages */
        $extractPages = $data['extract_pages'] ?? false;
        if (!is_bool($extractPages)) {
            /** @var bool $extractPages */
            $extractPages = (bool) $extractPages;
        }

        /** @var bool $insertPageMarkers */
        $insertPageMarkers = $data['insert_page_markers'] ?? false;
        if (!is_bool($insertPageMarkers)) {
            /** @var bool $insertPageMarkers */
            $insertPageMarkers = (bool) $insertPageMarkers;
        }

        /** @var string $markerFormat */
        $markerFormat = $data['marker_format'] ?? '--- Page {page_num} ---';
        if (!is_string($markerFormat)) {
            /** @var string $markerFormat */
            $markerFormat = (string) $markerFormat;
        }

        return new self(
            extractPages: $extractPages,
            insertPageMarkers: $insertPageMarkers,
            markerFormat: $markerFormat,
        );
    }

    /**
     * Create configuration from JSON string.
     */
    public static function fromJson(string $json): self
    {
        $data = json_decode($json, true);
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \InvalidArgumentException('Invalid JSON: ' . json_last_error_msg());
        }
        if (!is_array($data)) {
            throw new \InvalidArgumentException('JSON must decode to an object/array');
        }
        /** @var array<string, mixed> $data */
        return self::fromArray($data);
    }

    /**
     * Create configuration from JSON file.
     */
    public static function fromFile(string $path): self
    {
        if (!file_exists($path)) {
            throw new \InvalidArgumentException("File not found: {$path}");
        }
        $contents = file_get_contents($path);
        if ($contents === false) {
            throw new \InvalidArgumentException("Unable to read file: {$path}");
        }
        return self::fromJson($contents);
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'extract_pages' => $this->extractPages,
            'insert_page_markers' => $this->insertPageMarkers,
            'marker_format' => $this->markerFormat,
        ];
    }

    /**
     * Convert configuration to JSON string.
     */
    public function toJson(): string
    {
        $json = json_encode($this->toArray(), JSON_PRETTY_PRINT);
        if ($json === false) {
            throw new \RuntimeException('Failed to encode configuration to JSON');
        }
        return $json;
    }
}

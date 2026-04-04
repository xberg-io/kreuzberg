<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Configuration for OCR element extraction and processing.
 *
 * Controls how OCR elements are extracted, filtered, and organized
 * hierarchically within the document.
 */
readonly class OcrElementConfig
{
    public function __construct(
        /**
         * Enable extraction of individual OCR elements.
         *
         * When enabled, OCR results include detailed elements with positions,
         * confidence scores, and hierarchical relationships.
         *
         * @var bool
         * @default false
         */
        public bool $includeElements = false,

        /**
         * Minimum hierarchical level to include in element extraction.
         *
         * Filters which levels of text hierarchy to include. Valid values:
         * - 'page': Only page-level elements
         * - 'block': Page and block-level elements
         * - 'line': Page, block, and line-level elements
         * - 'word': All levels including individual words
         * - null: Include all levels (default)
         *
         * @var string|null
         * @default null
         */
        public ?string $minLevel = null,

        /**
         * Minimum confidence threshold for including elements.
         *
         * Excludes OCR elements with confidence scores below this threshold.
         * Range: 0.0 to 1.0, or null to include all elements regardless of confidence.
         *
         * @var float|null
         * @default null
         */
        public ?float $minConfidence = null,

        /**
         * Build element hierarchy relationships.
         *
         * When enabled, elements are organized into parent-child relationships
         * reflecting the document structure. When disabled, elements are listed
         * flat without hierarchy information.
         *
         * @var bool
         * @default false
         */
        public bool $buildHierarchy = false,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var bool $includeElements */
        $includeElements = $data['include_elements'] ?? false;
        if (!is_bool($includeElements)) {
            /** @var bool $includeElements */
            $includeElements = (bool) $includeElements;
        }

        /** @var string|null $minLevel */
        $minLevel = $data['min_level'] ?? null;
        if ($minLevel !== null && !is_string($minLevel)) {
            /** @var string $minLevel */
            $minLevel = (string) $minLevel;
        }

        /** @var float|null $minConfidence */
        $minConfidence = isset($data['min_confidence']) && is_numeric($data['min_confidence'])
            ? (float) $data['min_confidence']
            : null;

        /** @var bool $buildHierarchy */
        $buildHierarchy = $data['build_hierarchy'] ?? false;
        if (!is_bool($buildHierarchy)) {
            /** @var bool $buildHierarchy */
            $buildHierarchy = (bool) $buildHierarchy;
        }

        return new self(
            includeElements: $includeElements,
            minLevel: $minLevel,
            minConfidence: $minConfidence,
            buildHierarchy: $buildHierarchy,
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
        return array_filter([
            'include_elements' => $this->includeElements,
            'min_level' => $this->minLevel,
            'min_confidence' => $this->minConfidence,
            'build_hierarchy' => $this->buildHierarchy,
        ], static fn ($value): bool => $value !== null && $value !== false);
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

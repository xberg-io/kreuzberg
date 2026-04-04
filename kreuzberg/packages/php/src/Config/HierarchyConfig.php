<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Hierarchy detection configuration.
 */
readonly class HierarchyConfig
{
    public function __construct(
        /**
         * Enable hierarchical structure detection.
         *
         * When enabled, the extraction process analyzes document structure
         * and identifies heading hierarchies, section organization, and
         * content relationships to build a logical document tree.
         *
         * @var bool
         * @default true
         */
        public bool $enabled = true,

        /**
         * Number of clusters for hierarchical clustering.
         *
         * Used in hierarchical structure detection algorithms to group
         * related content elements. Higher values create finer-grained clusters,
         * lower values create broader groupings.
         *
         * Valid range: 2-20 (practical range: 2-10)
         * Recommended values:
         * - 2-4: For simple documents with few sections
         * - 4-6: Standard for typical documents
         * - 6-10: For complex documents with many subsections
         *
         * @var int
         * @default 6
         */
        public int $kClusters = 6,

        /**
         * Include bounding box coordinates in hierarchy results.
         *
         * When enabled, the bounding box (position and dimensions) for each
         * hierarchical element is included in results. Useful for reconstructing
         * original document layout or correlating with visual representation.
         *
         * @var bool
         * @default true
         */
        public bool $includeBbox = true,

        /**
         * OCR coverage threshold for hierarchy detection.
         *
         * Minimum fraction of text that must be successfully extracted via OCR
         * for OCR-specific hierarchical features to be used. Helps avoid relying
         * on OCR results when coverage is too low.
         *
         * Valid range: 0.0-1.0
         * Recommended values:
         * - 0.0: Use OCR results regardless of coverage
         * - 0.5: Require at least 50% OCR coverage
         * - 0.8-1.0: Strict coverage requirement for OCR-based features
         *
         * @var float|null
         * @default null (disabled)
         */
        public ?float $ocrCoverageThreshold = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var bool $enabled */
        $enabled = $data['enabled'] ?? true;
        if (!is_bool($enabled)) {
            /** @var bool $enabled */
            $enabled = (bool) $enabled;
        }

        /** @var int $kClusters */
        $kClusters = $data['k_clusters'] ?? 6;
        if (!is_int($kClusters)) {
            /** @var int $kClusters */
            $kClusters = (int) $kClusters;
        }

        /** @var bool $includeBbox */
        $includeBbox = $data['include_bbox'] ?? true;
        if (!is_bool($includeBbox)) {
            /** @var bool $includeBbox */
            $includeBbox = (bool) $includeBbox;
        }

        /** @var float|null $ocrCoverageThreshold */
        $ocrCoverageThreshold = $data['ocr_coverage_threshold'] ?? null;
        if ($ocrCoverageThreshold !== null && !is_float($ocrCoverageThreshold)) {
            /** @var float $ocrCoverageThreshold */
            $ocrCoverageThreshold = (float) $ocrCoverageThreshold;
        }

        return new self(
            enabled: $enabled,
            kClusters: $kClusters,
            includeBbox: $includeBbox,
            ocrCoverageThreshold: $ocrCoverageThreshold,
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
        $array = [
            'enabled' => $this->enabled,
            'k_clusters' => $this->kClusters,
            'include_bbox' => $this->includeBbox,
        ];

        if ($this->ocrCoverageThreshold !== null) {
            $array['ocr_coverage_threshold'] = $this->ocrCoverageThreshold;
        }

        return $array;
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

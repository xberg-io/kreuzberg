<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Language detection configuration.
 */
readonly class LanguageDetectionConfig
{
    public function __construct(
        /**
         * Enable automatic language detection.
         *
         * When enabled, the extraction process automatically detects the language(s)
         * of the document content and adjusts processing parameters accordingly.
         * Improves OCR accuracy and keyword extraction for multilingual documents.
         *
         * @var bool
         * @default false
         */
        public bool $enabled = false,

        /**
         * Maximum number of languages to detect in the document.
         *
         * Limits the number of distinct languages identified. Useful for preventing
         * false detection of language switches due to special terms or proper nouns
         * in mixed-language documents.
         *
         * Valid range: 1-unlimited (practical range: 1-10)
         * Recommended values:
         * - 1: For single-language documents only
         * - 2-3: For bilingual/multilingual documents
         * - null: Unlimited language detection
         *
         * @var int|null
         * @default null (unlimited)
         */
        public ?int $maxLanguages = null,

        /**
         * Minimum confidence threshold for language detection.
         *
         * Language detections with confidence below this threshold are excluded.
         * Higher thresholds reduce false positives but may miss legitimate languages.
         *
         * Valid range: 0.0-1.0
         * Recommended values:
         * - 0.0: Accept all detections regardless of confidence
         * - 0.5: Moderate confidence threshold
         * - 0.8-1.0: Strict confidence requirement
         *
         * @var float|null
         * @default null (no threshold)
         */
        public ?float $confidenceThreshold = null,
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
        $enabled = $data['enabled'] ?? false;
        if (!is_bool($enabled)) {
            /** @var bool $enabled */
            $enabled = (bool) $enabled;
        }

        /** @var int|null $maxLanguages */
        $maxLanguages = $data['max_languages'] ?? null;
        if ($maxLanguages !== null && !is_int($maxLanguages)) {
            /** @var int $maxLanguages */
            $maxLanguages = (int) $maxLanguages;
        }

        /** @var float|null $confidenceThreshold */
        $confidenceThreshold = $data['confidence_threshold'] ?? null;
        if ($confidenceThreshold !== null && !is_float($confidenceThreshold)) {
            /** @var float $confidenceThreshold */
            $confidenceThreshold = (float) $confidenceThreshold;
        }

        return new self(
            enabled: $enabled,
            maxLanguages: $maxLanguages,
            confidenceThreshold: $confidenceThreshold,
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
            'enabled' => $this->enabled,
            'max_languages' => $this->maxLanguages,
            'confidence_threshold' => $this->confidenceThreshold,
        ], static fn ($value): bool => $value !== null);
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

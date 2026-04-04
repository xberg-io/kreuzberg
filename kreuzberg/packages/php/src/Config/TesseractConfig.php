<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Tesseract OCR configuration.
 */
readonly class TesseractConfig
{
    public function __construct(
        /**
         * Page Segmentation Mode (PSM) for Tesseract.
         *
         * Tells Tesseract how to interpret page layout and segment the image
         * before OCR. Different modes work better for different document types
         * and layouts.
         *
         * Common modes:
         * - 0: Orientation and script detection only
         * - 1: Automatic page segmentation with OSD
         * - 3: Fully automatic page segmentation (default)
         * - 6: Assume a single uniform block of text
         * - 7: Treat as single text line
         * - 8: Treat as single word
         * - 11: Sparse text, find as much text as possible in no particular order
         * - 13: Raw line - treat the image as a single text line
         *
         * Valid range: 0-13
         *
         * @var int|null
         * @default null (Tesseract default, typically 3)
         * @example $config = new TesseractConfig(psm: 11);
         */
        public ?int $psm = null,

        /**
         * OCR Engine Mode (OEM) for Tesseract.
         *
         * Selects which OCR engine or combination of engines to use.
         *
         * Modes:
         * - 0: Legacy engine only
         * - 1: Neural net LSTM engine only
         * - 2: Legacy + LSTM engines (hybrid)
         * - 3: Default, automatic selection
         *
         * Valid range: 0-3
         *
         * @var int|null
         * @default null (Tesseract default, typically 3)
         */
        public ?int $oem = null,

        /**
         * Enable table detection in Tesseract.
         *
         * When enabled, Tesseract uses additional algorithms to detect
         * and parse table structures in the document. Improves text extraction
         * from documents with tabular data.
         *
         * @var bool
         * @default false
         */
        public bool $enableTableDetection = false,

        /**
         * Character whitelist for Tesseract.
         *
         * Restricts OCR to only recognize characters in this string.
         * Useful for documents that contain only known character sets
         * (e.g., digits only, specific symbols).
         *
         * Examples:
         * - '0123456789': Numbers only
         * - 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ': Letters only
         * - '0-9,-$': Numbers, dash, comma, dollar sign
         * - Leaving empty/null: No whitelist restriction
         *
         * @var string|null
         * @default null (no whitelist)
         */
        public ?string $tesseditCharWhitelist = null,

        /**
         * Character blacklist for Tesseract.
         *
         * Prevents OCR from recognizing characters in this string.
         * Useful for filtering out characters known to cause OCR errors.
         *
         * Examples:
         * - '|': Exclude pipe character (often confused with 'l')
         * - '~': Exclude tilde
         * - '|[]{}': Exclude bracket-like characters
         *
         * @var string|null
         * @default null (no blacklist)
         */
        public ?string $tesseditCharBlacklist = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int|null $psm */
        $psm = $data['psm'] ?? null;
        if ($psm !== null && !is_int($psm)) {
            /** @var int $psm */
            $psm = (int) $psm;
        }

        /** @var int|null $oem */
        $oem = $data['oem'] ?? null;
        if ($oem !== null && !is_int($oem)) {
            /** @var int $oem */
            $oem = (int) $oem;
        }

        /** @var bool $enableTableDetection */
        $enableTableDetection = $data['enable_table_detection'] ?? false;
        if (!is_bool($enableTableDetection)) {
            /** @var bool $enableTableDetection */
            $enableTableDetection = (bool) $enableTableDetection;
        }

        /** @var string|null $tesseditCharWhitelist */
        $tesseditCharWhitelist = $data['tessedit_char_whitelist'] ?? null;
        if ($tesseditCharWhitelist !== null && !is_string($tesseditCharWhitelist)) {
            /** @var string $tesseditCharWhitelist */
            $tesseditCharWhitelist = (string) $tesseditCharWhitelist;
        }

        /** @var string|null $tesseditCharBlacklist */
        $tesseditCharBlacklist = $data['tessedit_char_blacklist'] ?? null;
        if ($tesseditCharBlacklist !== null && !is_string($tesseditCharBlacklist)) {
            /** @var string $tesseditCharBlacklist */
            $tesseditCharBlacklist = (string) $tesseditCharBlacklist;
        }

        return new self(
            psm: $psm,
            oem: $oem,
            enableTableDetection: $enableTableDetection,
            tesseditCharWhitelist: $tesseditCharWhitelist,
            tesseditCharBlacklist: $tesseditCharBlacklist,
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
            'psm' => $this->psm,
            'oem' => $this->oem,
            'enable_table_detection' => $this->enableTableDetection,
            'tessedit_char_whitelist' => $this->tesseditCharWhitelist,
            'tessedit_char_blacklist' => $this->tesseditCharBlacklist,
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

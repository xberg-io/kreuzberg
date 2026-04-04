<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * PaddleOCR-specific configuration.
 *
 * Advanced settings for the PaddleOCR engine including language selection,
 * model caching, angle detection, table recognition, and detection thresholds.
 */
readonly class PaddleOcrConfig
{
    public function __construct(
        /**
         * Language code for OCR recognition.
         *
         * Specifies which language model to use for text recognition.
         * Common codes: 'ch' (Chinese), 'en' (English), 'fr' (French), etc.
         *
         * @var string|null
         * @default null
         */
        public ?string $language = null,

        /**
         * Directory for caching PaddleOCR models.
         *
         * Models are downloaded and cached in this directory to avoid repeated
         * downloads. If null, uses PaddleOCR's default cache location.
         *
         * @var string|null
         * @default null
         */
        public ?string $cacheDir = null,

        /**
         * Enable angle classification for rotated text.
         *
         * When enabled, uses an additional angle classification model to detect
         * and correct rotated text before recognition. Improves accuracy on rotated
         * documents but requires additional model loading time.
         *
         * @var bool|null
         * @default null
         */
        public ?bool $useAngleCls = null,

        /**
         * Enable table structure detection.
         *
         * When enabled, uses table detection models to identify and parse table
         * structures within the document. Improves extraction from documents with
         * tabular data.
         *
         * @var bool|null
         * @default null
         */
        public ?bool $enableTableDetection = null,

        /**
         * Threshold for text region detection.
         *
         * Controls the sensitivity of text detection. Lower values detect more regions
         * (more false positives), higher values detect fewer regions (more false negatives).
         * Range: 0.0 to 1.0
         *
         * @var float|null
         * @default null
         */
        public ?float $detDbThresh = null,

        /**
         * Box filter threshold for text detection.
         *
         * Filters out detected text boxes with scores below this threshold.
         * Range: 0.0 to 1.0
         *
         * @var float|null
         * @default null
         */
        public ?float $detDbBoxThresh = null,

        /**
         * Unclip ratio for expanding detected text regions.
         *
         * Controls how much to expand detected text boxes. Higher values create
         * larger boxes. Typically between 1.0 and 2.5.
         *
         * @var float|null
         * @default null
         */
        public ?float $detDbUnclipRatio = null,

        /**
         * Limit side length for detection image resize.
         *
         * Maximum side length for resizing detection input. If an image dimension
         * exceeds this, the image is scaled down. Larger values improve accuracy
         * but require more memory and computation.
         *
         * @var int|null
         * @default null
         */
        public ?int $detLimitSideLen = null,

        /**
         * Batch size for recognition.
         *
         * Number of text regions to process simultaneously during recognition.
         * Higher values are faster but require more memory.
         *
         * @var int|null
         * @default null
         */
        public ?int $recBatchNum = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string|null $language */
        $language = $data['language'] ?? null;
        if ($language !== null && !is_string($language)) {
            /** @var string $language */
            $language = (string) $language;
        }

        /** @var string|null $cacheDir */
        $cacheDir = $data['cache_dir'] ?? null;
        if ($cacheDir !== null && !is_string($cacheDir)) {
            /** @var string $cacheDir */
            $cacheDir = (string) $cacheDir;
        }

        /** @var bool|null $useAngleCls */
        $useAngleCls = $data['use_angle_cls'] ?? null;
        if ($useAngleCls !== null && !is_bool($useAngleCls)) {
            /** @var bool $useAngleCls */
            $useAngleCls = (bool) $useAngleCls;
        }

        /** @var bool|null $enableTableDetection */
        $enableTableDetection = $data['enable_table_detection'] ?? null;
        if ($enableTableDetection !== null && !is_bool($enableTableDetection)) {
            /** @var bool $enableTableDetection */
            $enableTableDetection = (bool) $enableTableDetection;
        }

        /** @var float|null $detDbThresh */
        $detDbThresh = isset($data['det_db_thresh']) && is_numeric($data['det_db_thresh'])
            ? (float) $data['det_db_thresh']
            : null;

        /** @var float|null $detDbBoxThresh */
        $detDbBoxThresh = isset($data['det_db_box_thresh']) && is_numeric($data['det_db_box_thresh'])
            ? (float) $data['det_db_box_thresh']
            : null;

        /** @var float|null $detDbUnclipRatio */
        $detDbUnclipRatio = isset($data['det_db_unclip_ratio']) && is_numeric($data['det_db_unclip_ratio'])
            ? (float) $data['det_db_unclip_ratio']
            : null;

        /** @var int|null $detLimitSideLen */
        $detLimitSideLen = isset($data['det_limit_side_len']) && is_numeric($data['det_limit_side_len'])
            ? (int) $data['det_limit_side_len']
            : null;

        /** @var int|null $recBatchNum */
        $recBatchNum = isset($data['rec_batch_num']) && is_numeric($data['rec_batch_num'])
            ? (int) $data['rec_batch_num']
            : null;

        return new self(
            language: $language,
            cacheDir: $cacheDir,
            useAngleCls: $useAngleCls,
            enableTableDetection: $enableTableDetection,
            detDbThresh: $detDbThresh,
            detDbBoxThresh: $detDbBoxThresh,
            detDbUnclipRatio: $detDbUnclipRatio,
            detLimitSideLen: $detLimitSideLen,
            recBatchNum: $recBatchNum,
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
            'language' => $this->language,
            'cache_dir' => $this->cacheDir,
            'use_angle_cls' => $this->useAngleCls,
            'enable_table_detection' => $this->enableTableDetection,
            'det_db_thresh' => $this->detDbThresh,
            'det_db_box_thresh' => $this->detDbBoxThresh,
            'det_db_unclip_ratio' => $this->detDbUnclipRatio,
            'det_limit_side_len' => $this->detLimitSideLen,
            'rec_batch_num' => $this->recBatchNum,
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

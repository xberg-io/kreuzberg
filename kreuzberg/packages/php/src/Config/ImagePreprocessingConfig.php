<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Image preprocessing configuration for OCR.
 *
 * Configuration class for controlling how images are preprocessed before
 * OCR processing. Provides settings for resolution scaling, rotation,
 * skew correction, and various image enhancement techniques.
 */
readonly class ImagePreprocessingConfig
{
    public function __construct(
        /**
         * Target DPI for image upscaling/downscaling.
         *
         * Adjusts image resolution to the specified dots-per-inch.
         * Higher DPI improves OCR accuracy but increases processing time.
         * Standard DPI for OCR is 300.
         *
         * Valid range: 50-600 DPI
         *
         * @var int
         * @default 300
         */
        public int $targetDpi = 300,

        /**
         * Auto-detect and correct image rotation.
         *
         * When enabled, automatically detects and corrects image rotation
         * to ensure text is properly oriented for OCR processing.
         *
         * @var bool
         * @default true
         */
        public bool $autoRotate = true,

        /**
         * Correct image skew (perspective distortion).
         *
         * When enabled, detects and corrects slight skewing or perspective
         * distortion in scanned documents. Improves OCR accuracy for
         * imperfectly scanned or photographed documents.
         *
         * @var bool
         * @default true
         */
        public bool $deskew = true,

        /**
         * Apply noise reduction/denoising to images.
         *
         * When enabled, reduces visual noise in images which can improve
         * OCR accuracy for low-quality scans, faxes, or damaged documents.
         *
         * @var bool
         * @default false
         */
        public bool $denoise = false,

        /**
         * Enhance contrast for better text visibility.
         *
         * When enabled, applies contrast enhancement to improve text
         * legibility, especially for faded or low-contrast documents.
         *
         * @var bool
         * @default false
         */
        public bool $contrastEnhance = false,

        /**
         * Binarization method for converting to black and white.
         *
         * Controls how grayscale images are converted to binary (black/white)
         * for OCR processing. Common values: "otsu", "sauvola", "adaptive".
         *
         * @var string
         * @default "otsu"
         */
        public string $binarizationMethod = 'otsu',

        /**
         * Invert colors (white text on black → black on white).
         *
         * When enabled, inverts the image colors. Useful for documents
         * with light text on dark backgrounds.
         *
         * @var bool
         * @default false
         */
        public bool $invertColors = false,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int $targetDpi */
        $targetDpi = $data['target_dpi'] ?? 300;
        if (!is_int($targetDpi)) {
            /** @var int $targetDpi */
            $targetDpi = (int) $targetDpi;
        }

        /** @var bool $autoRotate */
        $autoRotate = $data['auto_rotate'] ?? true;
        if (!is_bool($autoRotate)) {
            /** @var bool $autoRotate */
            $autoRotate = (bool) $autoRotate;
        }

        /** @var bool $deskew */
        $deskew = $data['deskew'] ?? true;
        if (!is_bool($deskew)) {
            /** @var bool $deskew */
            $deskew = (bool) $deskew;
        }

        /** @var bool $denoise */
        $denoise = $data['denoise'] ?? false;
        if (!is_bool($denoise)) {
            /** @var bool $denoise */
            $denoise = (bool) $denoise;
        }

        /** @var bool $contrastEnhance */
        $contrastEnhance = $data['contrast_enhance'] ?? false;
        if (!is_bool($contrastEnhance)) {
            /** @var bool $contrastEnhance */
            $contrastEnhance = (bool) $contrastEnhance;
        }

        /** @var string $binarizationMethod */
        $binarizationMethod = $data['binarization_method'] ?? 'otsu';
        if (!is_string($binarizationMethod)) {
            /** @var string $binarizationMethod */
            $binarizationMethod = (string) $binarizationMethod;
        }

        /** @var bool $invertColors */
        $invertColors = $data['invert_colors'] ?? false;
        if (!is_bool($invertColors)) {
            /** @var bool $invertColors */
            $invertColors = (bool) $invertColors;
        }

        return new self(
            targetDpi: $targetDpi,
            autoRotate: $autoRotate,
            deskew: $deskew,
            denoise: $denoise,
            contrastEnhance: $contrastEnhance,
            binarizationMethod: $binarizationMethod,
            invertColors: $invertColors,
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
            'target_dpi' => $this->targetDpi,
            'auto_rotate' => $this->autoRotate,
            'deskew' => $this->deskew,
            'denoise' => $this->denoise,
            'contrast_enhance' => $this->contrastEnhance,
            'binarization_method' => $this->binarizationMethod,
            'invert_colors' => $this->invertColors,
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

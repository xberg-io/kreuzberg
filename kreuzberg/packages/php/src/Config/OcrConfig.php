<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * OCR configuration.
 */
readonly class OcrConfig
{
    public function __construct(
        /**
         * OCR backend to use for text extraction.
         *
         * Selects which OCR engine to use for document processing.
         * Available backends:
         * - 'tesseract': Tesseract OCR engine (requires tesseract installation)
         * - 'paddle-ocr': PaddleOCR engine via ONNX Runtime
         *
         * @var string
         * @default 'tesseract'
         */
        public string $backend = 'tesseract',

        /**
         * Language for OCR text recognition.
         *
         * Specifies the language(s) to use for OCR. Use ISO 639-3 language codes.
         * Multiple languages can be specified as comma-separated values for mixed-language documents.
         * Examples: 'eng', 'deu', 'fra', 'eng+deu', 'eng+fra+deu'
         *
         * @var string
         * @default 'eng'
         */
        public string $language = 'eng',

        /**
         * Tesseract-specific OCR configuration.
         *
         * Contains advanced settings for Tesseract OCR engine including page segmentation mode,
         * engine mode, character whitelists/blacklists, and table detection options.
         *
         * @var TesseractConfig|null
         * @default null
         */
        public ?TesseractConfig $tesseractConfig = null,

        /**
         * PaddleOCR-specific configuration.
         *
         * Contains advanced settings for PaddleOCR engine including language selection,
         * model caching, angle detection, table recognition, and detection thresholds.
         *
         * @var PaddleOcrConfig|null
         * @default null
         */
        public ?PaddleOcrConfig $paddleOcrConfig = null,

        /**
         * Image preprocessing configuration for OCR.
         *
         * Configures preprocessing steps applied to images before OCR,
         * including rotation correction, deskewing, denoising, and contrast adjustment.
         *
         * @var ImagePreprocessingConfig|null
         * @default null
         */
        public ?ImagePreprocessingConfig $imagePreprocessing = null,

        /**
         * OCR element extraction configuration.
         *
         * Controls extraction of individual OCR elements with positions,
         * confidence scores, and hierarchical relationships.
         *
         * @var OcrElementConfig|null
         * @default null
         */
        public ?OcrElementConfig $elementConfig = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $backend */
        $backend = $data['backend'] ?? 'tesseract';
        if (!is_string($backend)) {
            /** @var string $backend */
            $backend = (string) $backend;
        }

        /** @var string $language */
        $language = $data['language'] ?? 'eng';
        if (!is_string($language)) {
            /** @var string $language */
            $language = (string) $language;
        }

        /** @var TesseractConfig|null $tesseractConfig */
        $tesseractConfig = null;
        if (isset($data['tesseract_config'])) {
            $configData = $data['tesseract_config'];
            if (!is_array($configData)) {
                /** @var array<string, mixed> $configData */
                $configData = (array) $configData;
            }
            /** @var array<string, mixed> $configData */
            $tesseractConfig = TesseractConfig::fromArray($configData);
        }

        /** @var PaddleOcrConfig|null $paddleOcrConfig */
        $paddleOcrConfig = null;
        if (isset($data['paddle_ocr_config'])) {
            $configData = $data['paddle_ocr_config'];
            if (!is_array($configData)) {
                /** @var array<string, mixed> $configData */
                $configData = (array) $configData;
            }
            /** @var array<string, mixed> $configData */
            $paddleOcrConfig = PaddleOcrConfig::fromArray($configData);
        }

        /** @var ImagePreprocessingConfig|null $imagePreprocessing */
        $imagePreprocessing = null;
        if (isset($data['image_preprocessing'])) {
            $configData = $data['image_preprocessing'];
            if (!is_array($configData)) {
                /** @var array<string, mixed> $configData */
                $configData = (array) $configData;
            }
            /** @var array<string, mixed> $configData */
            $imagePreprocessing = ImagePreprocessingConfig::fromArray($configData);
        }

        /** @var OcrElementConfig|null $elementConfig */
        $elementConfig = null;
        if (isset($data['element_config'])) {
            $configData = $data['element_config'];
            if (!is_array($configData)) {
                /** @var array<string, mixed> $configData */
                $configData = (array) $configData;
            }
            /** @var array<string, mixed> $configData */
            $elementConfig = OcrElementConfig::fromArray($configData);
        }

        return new self(
            backend: $backend,
            language: $language,
            tesseractConfig: $tesseractConfig ?? null,
            paddleOcrConfig: $paddleOcrConfig ?? null,
            imagePreprocessing: $imagePreprocessing ?? null,
            elementConfig: $elementConfig ?? null,
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
            'backend' => $this->backend,
            'language' => $this->language,
            'tesseract_config' => $this->tesseractConfig?->toArray(),
            'paddle_ocr_config' => $this->paddleOcrConfig?->toArray(),
            'image_preprocessing' => $this->imagePreprocessing?->toArray(),
            'element_config' => $this->elementConfig?->toArray(),
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

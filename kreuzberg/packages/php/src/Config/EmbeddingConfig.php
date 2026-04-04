<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Embedding generation configuration.
 */
readonly class EmbeddingConfig
{
    public function __construct(
        /**
         * Embedding preset name.
         *
         * Specifies which pre-configured embedding preset to use for generating
         * vector representations of text chunks. Each preset determines the embedding
         * model, dimension, quality, and processing speed.
         *
         * Available presets:
         * - 'fast': AllMiniLML6V2Q (384 dims) - Quick prototyping, low-latency
         * - 'balanced': BGEBaseENV15 (768 dims) - General-purpose RAG
         * - 'quality': BGELargeENV15 (1024 dims) - High-quality embeddings
         * - 'multilingual': MultilingualE5Base (768 dims) - Multi-language support
         *
         * @var string
         * @default 'balanced'
         * @example $config = new EmbeddingConfig(model: 'quality');
         */
        public string $model = 'balanced',

        /**
         * Normalize embedding vectors to unit length.
         *
         * When enabled, embeddings are normalized to have unit norm (length of 1).
         * This is beneficial for cosine similarity calculations and ensures
         * consistent similarity scoring across different documents.
         *
         * @var bool
         * @default true
         */
        public bool $normalize = true,

        /**
         * Batch size for embedding generation.
         *
         * Number of text chunks to process simultaneously when generating embeddings.
         * Larger batches improve processing speed but require more memory.
         * Smaller batches reduce memory usage but are slower.
         *
         * Valid range: 1-unlimited (practical range: 1-512)
         * Recommended values:
         * - 1-32: For memory-constrained environments
         * - 32-128: Standard batch sizes for most systems
         * - 128-512: For high-memory systems with GPU acceleration
         *
         * @var int|null
         * @default null (system default, typically 32)
         */
        public ?int $batchSize = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string|array<string, string> $model */
        $model = $data['model'] ?? 'balanced';
        if (is_array($model)) {
            // Handle Rust-format model: {"type": "preset", "name": "balanced"}
            /** @var string $model */
            $model = $model['name'] ?? 'balanced';
        } elseif (!is_string($model)) {
            /** @var string $model */
            $model = (string) $model;
        }

        /** @var bool $normalize */
        $normalize = $data['normalize'] ?? true;
        if (!is_bool($normalize)) {
            /** @var bool $normalize */
            $normalize = (bool) $normalize;
        }

        /** @var int|null $batchSize */
        $batchSize = $data['batch_size'] ?? null;
        if ($batchSize !== null && !is_int($batchSize)) {
            /** @var int $batchSize */
            $batchSize = (int) $batchSize;
        }

        return new self(
            model: $model,
            normalize: $normalize,
            batchSize: $batchSize,
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
            'model' => $this->model,
            'normalize' => $this->normalize,
            'batch_size' => $this->batchSize,
        ], static fn ($value): bool => $value !== null);
    }

    /**
     * Convert to Rust-compatible format for serialization.
     * Internal use only - converts the model string to Rust's EmbeddingModelType format.
     *
     * @return array<string, mixed>
     *
     * @internal
     */
    public function toRustArray(): array
    {
        return array_filter([
            'model' => [
                'type' => 'preset',
                'name' => $this->model,
            ],
            'normalize' => $this->normalize,
            'batch_size' => $this->batchSize,
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

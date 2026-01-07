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
         * Embedding model name or identifier.
         *
         * Specifies which pre-trained embedding model to use for generating
         * vector representations of text chunks. The model determines embedding
         * dimension, quality, and processing speed.
         *
         * Common models:
         * - 'all-MiniLM-L6-v2': Lightweight, fast (dimension: 384)
         * - 'all-MiniLM-L12-v2': Balanced quality/speed (dimension: 384)
         * - 'all-mpnet-base-v2': High quality (dimension: 768)
         * - 'paraphrase-MiniLM-L6-v2': Good for semantic similarity (dimension: 384)
         * - 'multi-qa-MiniLM-L6-cos-v1': Optimized for Q&A (dimension: 384)
         *
         * @var string
         * @default 'all-MiniLM-L6-v2'
         * @example $config = new EmbeddingConfig(model: 'all-mpnet-base-v2');
         */
        public string $model = 'all-MiniLM-L6-v2',

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
        /** @var string $model */
        $model = $data['model'] ?? 'all-MiniLM-L6-v2';
        if (!is_string($model)) {
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

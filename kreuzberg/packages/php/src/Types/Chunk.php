<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Text chunk with optional embedding vector.
 *
 * @property-read string $content Chunk text content
 * @property-read array<float>|null $embedding Embedding vector
 * @property-read ChunkMetadata $metadata Chunk metadata
 */
readonly class Chunk
{
    /**
     * @param array<float>|null $embedding
     */
    public function __construct(
        public string $content,
        public ?array $embedding,
        public ChunkMetadata $metadata,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $content */
        $content = $data['content'] ?? '';

        /** @var array<float>|null $embedding */
        $embedding = $data['embedding'] ?? null;

        /** @var array<string, mixed> $metadata */
        $metadata = $data['metadata'] ?? [];

        return new self(
            content: $content,
            embedding: $embedding,
            metadata: ChunkMetadata::fromArray($metadata),
        );
    }
}

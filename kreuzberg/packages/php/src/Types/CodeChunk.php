<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Code chunk with source span and optional parent context.
 *
 * @property-read string $content Chunk text content
 * @property-read string $language Programming language
 * @property-read CodeSpan $span Source span
 * @property-read CodeChunkContext|null $context Optional parent context
 */
readonly class CodeChunk
{
    public function __construct(
        public string $content,
        public string $language,
        public CodeSpan $span,
        public ?CodeChunkContext $context = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $content */
        $content = $data['content'] ?? '';

        /** @var string $language */
        $language = $data['language'] ?? '';

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        /** @var array<string, mixed>|null $contextData */
        $contextData = is_array($data['context'] ?? null) ? $data['context'] : null;

        return new self(
            content: $content,
            language: $language,
            span: CodeSpan::fromArray($spanData),
            context: $contextData !== null ? CodeChunkContext::fromArray($contextData) : null,
        );
    }
}

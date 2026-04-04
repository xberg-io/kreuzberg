<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Structural code element (function, class, method, etc.).
 *
 * @property-read string $kind Item kind (e.g. 'function', 'class', 'method')
 * @property-read string|null $name Item name
 * @property-read string|null $visibility Visibility modifier (e.g. 'public', 'private')
 * @property-read CodeSpan $span Source span
 * @property-read array<CodeStructureItem> $children Nested structure items
 * @property-read array<string> $decorators Decorators/annotations
 * @property-read string|null $docComment Associated doc comment
 * @property-read string|null $signature Function/method signature
 * @property-read CodeSpan|null $bodySpan Span of the body block
 */
readonly class CodeStructureItem
{
    /**
     * @param array<CodeStructureItem> $children
     * @param array<string> $decorators
     */
    public function __construct(
        public string $kind,
        public ?string $name,
        public ?string $visibility,
        public CodeSpan $span,
        public array $children,
        public array $decorators,
        public ?string $docComment = null,
        public ?string $signature = null,
        public ?CodeSpan $bodySpan = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $kind */
        $kind = $data['kind'] ?? '';

        /** @var string|null $name */
        $name = $data['name'] ?? null;

        /** @var string|null $visibility */
        $visibility = $data['visibility'] ?? null;

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        /** @var array<array<string, mixed>> $childrenData */
        $childrenData = $data['children'] ?? [];

        /** @var array<string> $decorators */
        $decorators = $data['decorators'] ?? [];

        /** @var string|null $docComment */
        $docComment = $data['doc_comment'] ?? null;

        /** @var string|null $signature */
        $signature = $data['signature'] ?? null;

        /** @var array<string, mixed>|null $bodySpanData */
        $bodySpanData = is_array($data['body_span'] ?? null) ? $data['body_span'] : null;

        return new self(
            kind: $kind,
            name: $name,
            visibility: $visibility,
            span: CodeSpan::fromArray($spanData),
            children: array_map(
                /** @param array<string, mixed> $child */
                static fn (array $child): CodeStructureItem => CodeStructureItem::fromArray($child),
                $childrenData,
            ),
            decorators: $decorators,
            docComment: $docComment,
            signature: $signature,
            bodySpan: $bodySpanData !== null ? CodeSpan::fromArray($bodySpanData) : null,
        );
    }
}

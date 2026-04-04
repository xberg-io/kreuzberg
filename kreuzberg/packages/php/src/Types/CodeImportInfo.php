<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Import statement information.
 *
 * @property-read string $source Import source/module path
 * @property-read array<string> $items Imported item names
 * @property-read string|null $alias Import alias
 * @property-read bool $isWildcard Whether this is a wildcard import
 * @property-read CodeSpan $span Source span
 */
readonly class CodeImportInfo
{
    /**
     * @param array<string> $items
     */
    public function __construct(
        public string $source,
        public array $items,
        public ?string $alias,
        public bool $isWildcard,
        public CodeSpan $span,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $source */
        $source = $data['source'] ?? '';

        /** @var array<string> $items */
        $items = $data['items'] ?? [];

        /** @var string|null $alias */
        $alias = $data['alias'] ?? null;

        /** @var bool $isWildcard */
        $isWildcard = $data['is_wildcard'] ?? false;

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        return new self(
            source: $source,
            items: $items,
            alias: $alias,
            isWildcard: $isWildcard,
            span: CodeSpan::fromArray($spanData),
        );
    }
}

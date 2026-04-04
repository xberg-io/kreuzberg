<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Symbol definition information.
 *
 * @property-read string $name Symbol name
 * @property-read string $kind Symbol kind (e.g. 'variable', 'constant')
 * @property-read string|null $typeAnnotation Type annotation if present
 * @property-read CodeSpan $span Source span
 */
readonly class CodeSymbolInfo
{
    public function __construct(
        public string $name,
        public string $kind,
        public ?string $typeAnnotation,
        public CodeSpan $span,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $name */
        $name = $data['name'] ?? '';

        /** @var string $kind */
        $kind = $data['kind'] ?? '';

        /** @var string|null $typeAnnotation */
        $typeAnnotation = $data['type_annotation'] ?? null;

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        return new self(
            name: $name,
            kind: $kind,
            typeAnnotation: $typeAnnotation,
            span: CodeSpan::fromArray($spanData),
        );
    }
}

<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Export statement information.
 *
 * @property-read string $name Exported item name
 * @property-read string $kind Export kind
 * @property-read CodeSpan $span Source span
 */
readonly class CodeExportInfo
{
    public function __construct(
        public string $name,
        public string $kind,
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

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        return new self(
            name: $name,
            kind: $kind,
            span: CodeSpan::fromArray($spanData),
        );
    }
}

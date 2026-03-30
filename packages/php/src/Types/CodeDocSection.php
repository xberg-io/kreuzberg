<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Section within a docstring.
 *
 * @property-read string $kind Section kind (e.g. 'param', 'returns', 'description')
 * @property-read string|null $name Section name (e.g. parameter name)
 * @property-read string $content Section content text
 */
readonly class CodeDocSection
{
    public function __construct(
        public string $kind,
        public ?string $name,
        public string $content,
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

        /** @var string $content */
        $content = $data['content'] ?? '';

        return new self(
            kind: $kind,
            name: $name,
            content: $content,
        );
    }
}

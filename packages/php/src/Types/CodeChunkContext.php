<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Context for a code chunk (parent scope information).
 *
 * @property-read string|null $parentName Name of the parent scope
 * @property-read string|null $parentKind Kind of the parent scope
 */
readonly class CodeChunkContext
{
    public function __construct(
        public ?string $parentName = null,
        public ?string $parentKind = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string|null $parentName */
        $parentName = $data['parent_name'] ?? null;

        /** @var string|null $parentKind */
        $parentKind = $data['parent_kind'] ?? null;

        return new self(
            parentName: $parentName,
            parentKind: $parentKind,
        );
    }
}

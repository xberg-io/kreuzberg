<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Footnote definition in Djot document.
 *
 * @property-read string $label Footnote identifier
 * @property-read string $content Footnote content
 */
readonly class Footnote
{
    public function __construct(
        public string $label,
        public string $content,
    ) {
    }

    /**
     * Create Footnote from array.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $label */
        $label = $data['label'] ?? '';

        /** @var string $content */
        $content = $data['content'] ?? '';

        return new self(
            label: $label,
            content: $content,
        );
    }
}

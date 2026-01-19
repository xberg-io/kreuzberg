<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Image with metadata in Djot document.
 *
 * @property-read string $url Image URL or reference
 * @property-read string|null $alt Alternative text
 * @property-read string|null $title Image title/caption
 * @property-read array<string, mixed>|null $attributes Image attributes
 */
readonly class DjotImage
{
    /**
     * @param array<string, mixed>|null $attributes
     */
    public function __construct(
        public string $url,
        public ?string $alt = null,
        public ?string $title = null,
        public ?array $attributes = null,
    ) {
    }

    /**
     * Create DjotImage from array.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $url */
        $url = $data['url'] ?? '';

        /** @var string|null $alt */
        $alt = $data['alt'] ?? null;

        /** @var string|null $title */
        $title = $data['title'] ?? null;

        /** @var array<string, mixed>|null $attributes */
        $attributes = $data['attributes'] ?? null;

        return new self(
            url: $url,
            alt: $alt,
            title: $title,
            attributes: $attributes,
        );
    }
}

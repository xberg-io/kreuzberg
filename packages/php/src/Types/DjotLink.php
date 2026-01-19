<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Link in Djot document.
 *
 * @property-read string $url Link URL
 * @property-read string $text Link text
 * @property-read string|null $title Link title
 * @property-read string|null $linkType Link type (internal, external, email, phone, footnote)
 */
readonly class DjotLink
{
    public function __construct(
        public string $url,
        public string $text,
        public ?string $title = null,
        public ?string $linkType = null,
    ) {
    }

    /**
     * Create DjotLink from array.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $url */
        $url = $data['url'] ?? '';

        /** @var string $text */
        $text = $data['text'] ?? '';

        /** @var string|null $title */
        $title = $data['title'] ?? null;

        /** @var string|null $linkType */
        $linkType = $data['link_type'] ?? null;

        return new self(
            url: $url,
            text: $text,
            title: $title,
            linkType: $linkType,
        );
    }
}

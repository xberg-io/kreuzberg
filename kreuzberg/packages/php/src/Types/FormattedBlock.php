<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Block-level element in a Djot document.
 *
 * Represents structural elements like headings, paragraphs, lists, code blocks, etc.
 *
 * @property-read string $blockType Type of block element
 * @property-read int|null $level Heading level (1-6) or nesting level for lists
 * @property-read string|null $content Text content for inline elements
 * @property-read array<FormattedBlock> $children Child blocks for list items and containers
 * @property-read array<string, mixed>|null $attributes HTML/CSS attributes (id, class, etc.)
 */
readonly class FormattedBlock
{
    /**
     * @param array<FormattedBlock> $children
     * @param array<string, mixed>|null $attributes
     */
    public function __construct(
        public string $blockType,
        public ?int $level = null,
        public ?string $content = null,
        public array $children = [],
        public ?array $attributes = null,
    ) {
    }

    /**
     * Create FormattedBlock from array.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $blockType */
        $blockType = $data['block_type'] ?? 'paragraph';

        /** @var int|null $level */
        $level = $data['level'] ?? null;

        /** @var string|null $content */
        $content = $data['content'] ?? null;

        $children = [];
        if (isset($data['children'])) {
            /** @var array<array<string, mixed>> $childrenData */
            $childrenData = $data['children'];
            $children = array_map(
                /** @param array<string, mixed> $child */
                static fn (array $child): FormattedBlock => self::fromArray($child),
                $childrenData,
            );
        }

        /** @var array<string, mixed>|null $attributes */
        $attributes = $data['attributes'] ?? null;

        return new self(
            blockType: $blockType,
            level: $level,
            content: $content,
            children: $children,
            attributes: $attributes,
        );
    }
}

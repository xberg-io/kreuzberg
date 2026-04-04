<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Docstring information with parsed sections.
 *
 * @property-read string $text Raw docstring text
 * @property-read string $format Docstring format (e.g. 'javadoc', 'numpy', 'google')
 * @property-read string|null $associatedItem Name of the associated code item
 * @property-read CodeSpan $span Source span
 * @property-read array<CodeDocSection> $sections Parsed docstring sections
 */
readonly class CodeDocstringInfo
{
    /**
     * @param array<CodeDocSection> $sections
     */
    public function __construct(
        public string $text,
        public string $format,
        public ?string $associatedItem,
        public CodeSpan $span,
        public array $sections,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $text */
        $text = $data['text'] ?? '';

        /** @var string $format */
        $format = $data['format'] ?? '';

        /** @var string|null $associatedItem */
        $associatedItem = $data['associated_item'] ?? null;

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        /** @var array<array<string, mixed>> $sectionsData */
        $sectionsData = $data['sections'] ?? [];

        return new self(
            text: $text,
            format: $format,
            associatedItem: $associatedItem,
            span: CodeSpan::fromArray($spanData),
            sections: array_map(
                /** @param array<string, mixed> $section */
                static fn (array $section): CodeDocSection => CodeDocSection::fromArray($section),
                $sectionsData,
            ),
        );
    }
}

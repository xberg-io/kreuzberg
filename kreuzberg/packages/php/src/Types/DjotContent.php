<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Structured Djot document representation.
 *
 * Provides rich, block-level document structure with formatting,
 * tables, images, links, and metadata extracted from source documents.
 *
 * @property-read string $plainText Plain text representation for backward compatibility
 * @property-read array<FormattedBlock> $blocks Structured block-level content
 * @property-read Metadata $metadata Metadata from YAML frontmatter
 * @property-read array<Table> $tables Extracted tables as structured data
 * @property-read array<DjotImage> $images Extracted images with metadata
 * @property-read array<DjotLink> $links Extracted links with URLs and titles
 * @property-read array<Footnote> $footnotes Footnote definitions
 * @property-read array<string, mixed>|null $attributes Attributes mapped by element identifier
 */
readonly class DjotContent
{
    /**
     * @param array<FormattedBlock> $blocks
     * @param array<Table> $tables
     * @param array<DjotImage> $images
     * @param array<DjotLink> $links
     * @param array<Footnote> $footnotes
     * @param array<string, mixed>|null $attributes
     */
    public function __construct(
        public string $plainText,
        public array $blocks,
        public Metadata $metadata,
        public array $tables = [],
        public array $images = [],
        public array $links = [],
        public array $footnotes = [],
        public ?array $attributes = null,
    ) {
    }

    /**
     * Create DjotContent from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $plainText */
        $plainText = $data['plain_text'] ?? '';

        /** @var array<string, mixed> $metadataData */
        $metadataData = $data['metadata'] ?? [];

        /** @var array<array<string, mixed>> $blocksData */
        $blocksData = $data['blocks'] ?? [];

        $blocks = array_map(
            /** @param array<string, mixed> $block */
            static fn (array $block): FormattedBlock => FormattedBlock::fromArray($block),
            $blocksData,
        );

        /** @var array<array<string, mixed>> $tablesData */
        $tablesData = $data['tables'] ?? [];

        $tables = array_map(
            /** @param array<string, mixed> $table */
            static fn (array $table): Table => Table::fromArray($table),
            $tablesData,
        );

        $images = [];
        if (isset($data['images'])) {
            /** @var array<array<string, mixed>> $imagesData */
            $imagesData = $data['images'];
            $images = array_map(
                /** @param array<string, mixed> $image */
                static fn (array $image): DjotImage => DjotImage::fromArray($image),
                $imagesData,
            );
        }

        $links = [];
        if (isset($data['links'])) {
            /** @var array<array<string, mixed>> $linksData */
            $linksData = $data['links'];
            $links = array_map(
                /** @param array<string, mixed> $link */
                static fn (array $link): DjotLink => DjotLink::fromArray($link),
                $linksData,
            );
        }

        $footnotes = [];
        if (isset($data['footnotes'])) {
            /** @var array<array<string, mixed>> $footnotesData */
            $footnotesData = $data['footnotes'];
            $footnotes = array_map(
                /** @param array<string, mixed> $footnote */
                static fn (array $footnote): Footnote => Footnote::fromArray($footnote),
                $footnotesData,
            );
        }

        /** @var array<string, mixed>|null $attributes */
        $attributes = $data['attributes'] ?? null;

        return new self(
            plainText: $plainText,
            blocks: $blocks,
            metadata: Metadata::fromArray($metadataData),
            tables: $tables,
            images: $images,
            links: $links,
            footnotes: $footnotes,
            attributes: $attributes,
        );
    }
}

<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Page hierarchy structure containing heading levels and block information.
 *
 * @property-read int $blockCount Number of hierarchy blocks
 * @property-read array<HierarchicalBlock> $blocks Hierarchical blocks
 */
readonly class PageHierarchy
{
    /**
     * @param array<HierarchicalBlock> $blocks
     */
    public function __construct(
        public int $blockCount,
        public array $blocks = [],
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $blockCount = isset($data['block_count']) && is_int($data['block_count']) ? $data['block_count'] : 0;

        /** @var array<array<string, mixed>> $blocksData */
        $blocksData = $data['blocks'] ?? [];

        return new self(
            blockCount: $blockCount,
            blocks: array_map(
                /** @param array<string, mixed> $block */
                static fn (array $block): HierarchicalBlock => HierarchicalBlock::fromArray($block),
                $blocksData,
            ),
        );
    }
}

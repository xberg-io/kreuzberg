<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * A text block with hierarchy level assignment.
 *
 * @property-read string $text The text content of this block
 * @property-read float $fontSize The font size of the text
 * @property-read string $level The hierarchy level (h1-h6 or body)
 * @property-read ?array<float> $bbox Bounding box (left, top, right, bottom)
 */
readonly class HierarchicalBlock
{
    /**
     * @param ?array<float> $bbox
     */
    public function __construct(
        public string $text,
        public float $fontSize,
        public string $level,
        public ?array $bbox = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $text = isset($data['text']) && is_string($data['text']) ? $data['text'] : '';
        $fontSize = isset($data['font_size']) ? (is_float($data['font_size']) || is_int($data['font_size']) ? (float) $data['font_size'] : 0.0) : 0.0;
        $level = isset($data['level']) && is_string($data['level']) ? $data['level'] : 'body';
        $bboxRaw = $data['bbox'] ?? null;

        /** @var ?array<float> $bbox */
        $bbox = is_array($bboxRaw) ? array_map(static fn (mixed $v): float => is_float($v) || is_int($v) ? (float) $v : 0.0, $bboxRaw) : null;

        return new self(
            text: $text,
            fontSize: $fontSize,
            level: $level,
            bbox: $bbox,
        );
    }
}

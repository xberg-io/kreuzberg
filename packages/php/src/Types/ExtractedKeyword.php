<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Extracted keyword with relevance score and algorithm metadata.
 *
 * @property-read string $text Keyword text
 * @property-read float $score Keyword relevance score (0.0 to 1.0)
 * @property-read string $algorithm Algorithm used to extract this keyword
 * @property-read array<int>|null $positions Character offsets where keyword appears in text
 */
readonly class ExtractedKeyword
{
    /**
     * @param array<int>|null $positions
     */
    public function __construct(
        public string $text,
        public float $score,
        public string $algorithm,
        public ?array $positions = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $text */
        $text = $data['text'] ?? '';

        $score = 0.0;
        if (isset($data['score'])) {
            $value = $data['score'];
            if (is_numeric($value)) {
                $score = (float) $value;
            }
        }

        /** @var string $algorithm */
        $algorithm = $data['algorithm'] ?? '';

        /** @var array<int>|null $positions */
        $positions = $data['positions'] ?? null;

        return new self(
            text: $text,
            score: $score,
            algorithm: $algorithm,
            positions: $positions,
        );
    }
}

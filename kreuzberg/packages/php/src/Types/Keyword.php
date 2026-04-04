<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Extracted keyword with score.
 *
 * @property-read string $text Keyword text
 * @property-read float $score Keyword relevance score (0.0 to 1.0)
 * @property-read string|null $algorithm Algorithm used to extract this keyword ("yake" or "rake")
 * @property-read array<int>|null $positions Character offsets where keyword appears in text
 */
readonly class Keyword
{
    /**
     * @param array<int>|null $positions
     */
    public function __construct(
        public string $text,
        public float $score,
        public ?string $algorithm = null,
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

        /** @var string|null $algorithm */
        $algorithm = $data['algorithm'] ?? null;

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

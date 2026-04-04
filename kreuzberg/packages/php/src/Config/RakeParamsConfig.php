<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Algorithm-specific parameters for the RAKE keyword extraction algorithm.
 */
readonly class RakeParamsConfig
{
    public function __construct(
        /**
         * Minimum word length for keyword candidates.
         *
         * @var int|null
         * @default null (use algorithm default)
         */
        public ?int $minWordLength = null,

        /**
         * Maximum number of words per extracted phrase.
         *
         * @var int|null
         * @default null (use algorithm default)
         */
        public ?int $maxWordsPerPhrase = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $minWordLength = $data['min_word_length'] ?? null;
        $maxWordsPerPhrase = $data['max_words_per_phrase'] ?? null;

        return new self(
            minWordLength: is_int($minWordLength) ? $minWordLength : null,
            maxWordsPerPhrase: is_int($maxWordsPerPhrase) ? $maxWordsPerPhrase : null,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'min_word_length' => $this->minWordLength,
            'max_words_per_phrase' => $this->maxWordsPerPhrase,
        ], static fn ($value): bool => $value !== null);
    }
}

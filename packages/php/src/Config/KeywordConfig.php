<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Keyword extraction configuration.
 */
readonly class KeywordConfig
{
    public function __construct(
        /**
         * Algorithm to use for keyword extraction.
         *
         * Supported algorithms:
         * - 'yake': Yet Another Keyword Extractor (statistical approach)
         * - 'rake': Rapid Automatic Keyword Extraction (co-occurrence based)
         *
         * @var string
         * @default 'yake'
         */
        public string $algorithm = 'yake',

        /**
         * Maximum number of keywords to extract.
         *
         * Limits the number of top-ranked keywords returned from the extraction process.
         * Keywords are ranked by relevance score and only the top N keywords are returned.
         *
         * Valid range: 1-unlimited (practical range: 1-100)
         * Recommended values:
         * - 5-10: For concise summaries
         * - 10-20: For moderate detail
         * - 20-50: For comprehensive keyword extraction
         *
         * @var int
         * @default 10
         */
        public int $maxKeywords = 10,

        /**
         * Minimum relevance score threshold for keywords.
         *
         * Keywords with relevance scores below this threshold are excluded
         * from results. Score is typically normalized between 0 and 1.
         * Higher thresholds result in fewer, more relevant keywords.
         *
         * Valid range: 0.0-1.0
         * Recommended values:
         * - 0.0: Include all detected keywords
         * - 0.1-0.3: Moderate filtering
         * - 0.5+: Strict filtering for high-confidence keywords only
         *
         * @var float
         * @default 0.0
         */
        public float $minScore = 0.0,

        /**
         * Language for keyword extraction.
         *
         * Specifies the language context for keyword analysis and extraction.
         * Language-specific processing improves keyword quality by understanding
         * language nuances and importance of different terms.
         *
         * Common values: 'en', 'de', 'fr', 'es', 'it', 'pt', 'nl', 'ru', 'zh'
         *
         * @var string|null
         * @default 'en'
         */
        public ?string $language = 'en',

        /**
         * N-gram range for keyword extraction (min, max).
         *
         * Defines the range of n-grams to consider:
         * - [1, 1]: unigrams only (single words)
         * - [1, 2]: unigrams and bigrams (1-2 word phrases)
         * - [1, 3]: unigrams, bigrams, and trigrams (1-3 word phrases)
         *
         * @var array{int, int}
         * @default [1, 3]
         */
        public array $ngramRange = [1, 3],
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $algorithm */
        $algorithm = $data['algorithm'] ?? 'yake';
        if (!is_string($algorithm)) {
            /** @var string $algorithm */
            $algorithm = (string) $algorithm;
        }

        /** @var int $maxKeywords */
        $maxKeywords = $data['max_keywords'] ?? 10;
        if (!is_int($maxKeywords)) {
            /** @var int $maxKeywords */
            $maxKeywords = (int) $maxKeywords;
        }

        /** @var float $minScore */
        $minScore = $data['min_score'] ?? 0.0;
        if (!is_float($minScore) && !is_int($minScore)) {
            /** @var float $minScore */
            $minScore = (float) $minScore;
        }

        /** @var string|null $language */
        $language = $data['language'] ?? 'en';
        if (!is_string($language)) {
            /** @var string $language */
            $language = (string) $language;
        }

        /** @var array{int, int} $ngramRange */
        $ngramRange = $data['ngram_range'] ?? [1, 3];
        if (!is_array($ngramRange) || count($ngramRange) !== 2) {
            $ngramRange = [1, 3];
        }

        return new self(
            algorithm: $algorithm,
            maxKeywords: $maxKeywords,
            minScore: (float) $minScore,
            language: $language,
            ngramRange: $ngramRange,
        );
    }

    /**
     * Create configuration from JSON string.
     */
    public static function fromJson(string $json): self
    {
        $data = json_decode($json, true);
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \InvalidArgumentException('Invalid JSON: ' . json_last_error_msg());
        }
        if (!is_array($data)) {
            throw new \InvalidArgumentException('JSON must decode to an object/array');
        }
        /** @var array<string, mixed> $data */
        return self::fromArray($data);
    }

    /**
     * Create configuration from JSON file.
     */
    public static function fromFile(string $path): self
    {
        if (!file_exists($path)) {
            throw new \InvalidArgumentException("File not found: {$path}");
        }
        $contents = file_get_contents($path);
        if ($contents === false) {
            throw new \InvalidArgumentException("Unable to read file: {$path}");
        }
        return self::fromJson($contents);
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'algorithm' => $this->algorithm,
            'max_keywords' => $this->maxKeywords,
            'min_score' => $this->minScore,
            'language' => $this->language,
            'ngram_range' => $this->ngramRange,
        ], static fn ($value): bool => $value !== null);
    }

    /**
     * Convert configuration to JSON string.
     */
    public function toJson(): string
    {
        $json = json_encode($this->toArray(), JSON_PRETTY_PRINT);
        if ($json === false) {
            throw new \RuntimeException('Failed to encode configuration to JSON');
        }
        return $json;
    }
}

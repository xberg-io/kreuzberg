<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Token reduction configuration.
 */
readonly class TokenReductionConfig
{
    public function __construct(
        /**
         * Token reduction mode.
         *
         * Specifies the strategy for reducing the number of tokens in the extracted
         * content. Useful for staying within token limits of language models or
         * reducing processing costs while maintaining semantic content.
         *
         * Available modes:
         * - 'off': No token reduction, keep all content
         * - 'aggressive': Remove less important words and details
         * - 'moderate': Balanced reduction preserving most semantic content
         * - 'conservative': Minimal reduction, keep critical content only
         * - 'summarize': Summarize content instead of selective word removal
         *
         * @var string
         * @default 'off'
         */
        public string $mode = 'off',

        /**
         * Preserve important words during token reduction.
         *
         * When enabled, word importance analysis is performed to identify
         * and protect semantically significant terms (keywords, named entities, etc.)
         * from removal during token reduction. Ensures key information is retained.
         *
         * @var bool
         * @default true
         */
        public bool $preserveImportantWords = true,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $mode */
        $mode = $data['mode'] ?? 'off';
        if (!is_string($mode)) {
            /** @var string $mode */
            $mode = (string) $mode;
        }

        /** @var bool $preserveImportantWords */
        $preserveImportantWords = $data['preserve_important_words'] ?? true;
        if (!is_bool($preserveImportantWords)) {
            /** @var bool $preserveImportantWords */
            $preserveImportantWords = (bool) $preserveImportantWords;
        }

        return new self(
            mode: $mode,
            preserveImportantWords: $preserveImportantWords,
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
        return [
            'mode' => $this->mode,
            'preserve_important_words' => $this->preserveImportantWords,
        ];
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

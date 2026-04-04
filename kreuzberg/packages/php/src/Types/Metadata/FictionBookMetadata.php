<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * FictionBook (FB2) metadata.
 *
 * Contains genre, sequence, and annotation information from FB2 files.
 */
readonly class FictionBookMetadata
{
    /**
     * @param string[] $genres
     * @param string[] $sequences
     */
    public function __construct(
        public array $genres = [],
        public array $sequences = [],
        public ?string $annotation = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string[] $genres */
        $genres = $data['genres'] ?? [];
        if (!is_array($genres)) {
            $genres = [];
        }

        /** @var string[] $sequences */
        $sequences = $data['sequences'] ?? [];
        if (!is_array($sequences)) {
            $sequences = [];
        }

        /** @var string|null $annotation */
        $annotation = $data['annotation'] ?? null;

        return new self(
            genres: $genres,
            sequences: $sequences,
            annotation: $annotation,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [];

        if ($this->genres !== []) {
            $result['genres'] = $this->genres;
        }

        if ($this->sequences !== []) {
            $result['sequences'] = $this->sequences;
        }

        if ($this->annotation !== null) {
            $result['annotation'] = $this->annotation;
        }

        return $result;
    }
}

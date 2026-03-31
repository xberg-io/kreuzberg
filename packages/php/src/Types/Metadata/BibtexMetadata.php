<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * BibTeX bibliography metadata.
 *
 * Contains information about BibTeX entries including citation keys,
 * authors, year ranges, and entry type distribution.
 */
readonly class BibtexMetadata
{
    /**
     * @param string[] $citationKeys
     * @param string[] $authors
     * @param array<string, int>|null $entryTypes
     */
    public function __construct(
        public int $entryCount = 0,
        public array $citationKeys = [],
        public array $authors = [],
        public ?YearRange $yearRange = null,
        public ?array $entryTypes = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawEntryCount = $data['entry_count'] ?? 0;
        $entryCount = is_int($rawEntryCount) ? $rawEntryCount : (is_numeric($rawEntryCount) ? (int) $rawEntryCount : 0);

        /** @var string[] $citationKeys */
        $citationKeys = $data['citation_keys'] ?? [];
        if (!is_array($citationKeys)) {
            $citationKeys = [];
        }

        /** @var string[] $authors */
        $authors = $data['authors'] ?? [];
        if (!is_array($authors)) {
            $authors = [];
        }

        $yearRange = null;
        $yearRangeRaw = $data['year_range'] ?? null;
        if (is_array($yearRangeRaw)) {
            /** @var array<string, mixed> $yearRangeData */
            $yearRangeData = $yearRangeRaw;
            $yearRange = YearRange::fromArray($yearRangeData);
        }

        /** @var array<string, int>|null $entryTypes */
        $entryTypes = $data['entry_types'] ?? null;
        if (!is_array($entryTypes)) {
            $entryTypes = null;
        }

        return new self(
            entryCount: $entryCount,
            citationKeys: $citationKeys,
            authors: $authors,
            yearRange: $yearRange,
            entryTypes: $entryTypes,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [
            'entry_count' => $this->entryCount,
        ];

        if ($this->citationKeys !== []) {
            $result['citation_keys'] = $this->citationKeys;
        }

        if ($this->authors !== []) {
            $result['authors'] = $this->authors;
        }

        if ($this->yearRange !== null) {
            $result['year_range'] = $this->yearRange->toArray();
        }

        if ($this->entryTypes !== null) {
            $result['entry_types'] = $this->entryTypes;
        }

        return $result;
    }
}

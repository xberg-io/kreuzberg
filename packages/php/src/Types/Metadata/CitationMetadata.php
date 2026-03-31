<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * Citation file metadata (RIS, PubMed, EndNote).
 *
 * Contains information about citation records including format,
 * authors, year ranges, DOIs, and keywords.
 */
readonly class CitationMetadata
{
    /**
     * @param string[] $authors
     * @param string[] $dois
     * @param string[] $keywords
     */
    public function __construct(
        public int $citationCount = 0,
        public ?string $format = null,
        public array $authors = [],
        public ?YearRange $yearRange = null,
        public array $dois = [],
        public array $keywords = [],
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawCitationCount = $data['citation_count'] ?? 0;
        $citationCount = is_int($rawCitationCount) ? $rawCitationCount : (is_numeric($rawCitationCount) ? (int) $rawCitationCount : 0);

        /** @var string|null $format */
        $format = $data['format'] ?? null;

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

        /** @var string[] $dois */
        $dois = $data['dois'] ?? [];
        if (!is_array($dois)) {
            $dois = [];
        }

        /** @var string[] $keywords */
        $keywords = $data['keywords'] ?? [];
        if (!is_array($keywords)) {
            $keywords = [];
        }

        return new self(
            citationCount: $citationCount,
            format: $format,
            authors: $authors,
            yearRange: $yearRange,
            dois: $dois,
            keywords: $keywords,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [
            'citation_count' => $this->citationCount,
        ];

        if ($this->format !== null) {
            $result['format'] = $this->format;
        }

        if ($this->authors !== []) {
            $result['authors'] = $this->authors;
        }

        if ($this->yearRange !== null) {
            $result['year_range'] = $this->yearRange->toArray();
        }

        if ($this->dois !== []) {
            $result['dois'] = $this->dois;
        }

        if ($this->keywords !== []) {
            $result['keywords'] = $this->keywords;
        }

        return $result;
    }
}

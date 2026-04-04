<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * EPUB metadata (Dublin Core extensions).
 *
 * Contains extended Dublin Core metadata fields specific to EPUB files.
 */
readonly class EpubMetadata
{
    public function __construct(
        public ?string $coverage = null,
        public ?string $dcFormat = null,
        public ?string $relation = null,
        public ?string $source = null,
        public ?string $dcType = null,
        public ?string $coverImage = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawCoverage = $data['coverage'] ?? null;
        $rawDcFormat = $data['dc_format'] ?? null;
        $rawRelation = $data['relation'] ?? null;
        $rawSource = $data['source'] ?? null;
        $rawDcType = $data['dc_type'] ?? null;
        $rawCoverImage = $data['cover_image'] ?? null;

        return new self(
            coverage: is_string($rawCoverage) ? $rawCoverage : null,
            dcFormat: is_string($rawDcFormat) ? $rawDcFormat : null,
            relation: is_string($rawRelation) ? $rawRelation : null,
            source: is_string($rawSource) ? $rawSource : null,
            dcType: is_string($rawDcType) ? $rawDcType : null,
            coverImage: is_string($rawCoverImage) ? $rawCoverImage : null,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [];

        if ($this->coverage !== null) {
            $result['coverage'] = $this->coverage;
        }

        if ($this->dcFormat !== null) {
            $result['dc_format'] = $this->dcFormat;
        }

        if ($this->relation !== null) {
            $result['relation'] = $this->relation;
        }

        if ($this->source !== null) {
            $result['source'] = $this->source;
        }

        if ($this->dcType !== null) {
            $result['dc_type'] = $this->dcType;
        }

        if ($this->coverImage !== null) {
            $result['cover_image'] = $this->coverImage;
        }

        return $result;
    }
}

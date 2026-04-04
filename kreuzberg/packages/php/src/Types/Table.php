<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Extracted table structure.
 *
 * @property-read array<array<string>> $cells Table cells (2D array)
 * @property-read string $markdown Table in markdown format
 * @property-read int $pageNumber Page number where table was found
 * @property-read BoundingBox|null $boundingBox Bounding box coordinates if available
 */
readonly class Table
{
    /**
     * @param array<array<string>> $cells
     */
    public function __construct(
        public array $cells,
        public string $markdown,
        public int $pageNumber,
        public ?BoundingBox $boundingBox = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var array<array<string>> $cells */
        $cells = $data['cells'] ?? [];

        /** @var string $markdown */
        $markdown = $data['markdown'] ?? '';

        /** @var int $pageNumber */
        $pageNumber = $data['page_number'] ?? 0;

        $boundingBox = null;
        if (isset($data['bounding_box'])) {
            /** @var array<string, mixed> $boundingBoxData */
            $boundingBoxData = $data['bounding_box'];
            $boundingBox = BoundingBox::fromArray($boundingBoxData);
        }

        return new self(
            cells: $cells,
            markdown: $markdown,
            pageNumber: $pageNumber,
            boundingBox: $boundingBox,
        );
    }
}

<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * A PDF annotation extracted from a document page.
 *
 * @property-read string $annotationType The type of annotation
 * @property-read string|null $content Text content of the annotation
 * @property-read int $pageNumber Page number where the annotation appears (1-indexed)
 * @property-read array<string, float>|null $boundingBox Bounding box coordinates
 */
readonly class PdfAnnotation
{
    /**
     * @param array<string, float>|null $boundingBox
     */
    public function __construct(
        public string $annotationType,
        public ?string $content,
        public int $pageNumber,
        public ?array $boundingBox = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $annotationType */
        $annotationType = $data['annotation_type'] ?? '';

        /** @var string|null $content */
        $content = $data['content'] ?? null;

        $pageNumber = isset($data['page_number']) && is_numeric($data['page_number'])
            ? (int) $data['page_number']
            : 0;

        /** @var array<string, float>|null $boundingBox */
        $boundingBox = isset($data['bounding_box']) && is_array($data['bounding_box'])
            ? $data['bounding_box']
            : null;

        return new self(
            annotationType: $annotationType,
            content: $content,
            pageNumber: $pageNumber,
            boundingBox: $boundingBox,
        );
    }
}

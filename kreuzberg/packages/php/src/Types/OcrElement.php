<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * OCR-extracted text element with detailed positioning and confidence information.
 *
 * Represents a text element detected by OCR with its content, geometry,
 * confidence scores, and hierarchical relationships within a document.
 *
 * @property-read string $text The recognized text content
 * @property-read OcrBoundingGeometry|null $geometry Bounding geometry for the element
 * @property-read OcrConfidence|null $confidence Confidence scores for detection and recognition
 * @property-read string|null $level Hierarchical level of the element (e.g., "page", "block", "line", "word")
 * @property-read OcrRotation|null $rotation Rotation information for the text
 * @property-read int|null $pageNumber Page number where element appears (1-indexed)
 * @property-read string|null $parentId ID of parent element in hierarchy
 * @property-read array<string, mixed>|null $backendMetadata Backend-specific metadata
 */
readonly class OcrElement
{
    /**
     * @param array<string, mixed>|null $backendMetadata
     */
    public function __construct(
        public string $text = '',
        public ?OcrBoundingGeometry $geometry = null,
        public ?OcrConfidence $confidence = null,
        public ?string $level = null,
        public ?OcrRotation $rotation = null,
        public ?int $pageNumber = null,
        public ?string $parentId = null,
        public ?array $backendMetadata = null,
    ) {
    }

    /**
     * Create OcrElement from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $text */
        $text = $data['text'] ?? '';

        $geometry = null;
        if (isset($data['geometry'])) {
            /** @var array<string, mixed> $geometryData */
            $geometryData = $data['geometry'];
            $geometry = OcrBoundingGeometry::fromArray($geometryData);
        }

        $confidence = null;
        if (isset($data['confidence'])) {
            /** @var array<string, mixed> $confidenceData */
            $confidenceData = $data['confidence'];
            $confidence = OcrConfidence::fromArray($confidenceData);
        }

        /** @var string|null $level */
        $level = $data['level'] ?? null;

        $rotation = null;
        if (isset($data['rotation'])) {
            /** @var array<string, mixed> $rotationData */
            $rotationData = $data['rotation'];
            $rotation = OcrRotation::fromArray($rotationData);
        }

        /** @var int|null $pageNumber */
        $pageNumber = isset($data['page_number']) && is_numeric($data['page_number'])
            ? (int) $data['page_number']
            : null;

        /** @var string|null $parentId */
        $parentId = $data['parent_id'] ?? null;

        /** @var array<string, mixed>|null $backendMetadata */
        $backendMetadata = $data['backend_metadata'] ?? null;

        return new self(
            text: $text,
            geometry: $geometry,
            confidence: $confidence,
            level: $level,
            rotation: $rotation,
            pageNumber: $pageNumber,
            parentId: $parentId,
            backendMetadata: $backendMetadata,
        );
    }
}

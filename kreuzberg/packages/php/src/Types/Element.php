<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Bounding box coordinates for element positioning.
 *
 * @property-read float $x0 Left x-coordinate
 * @property-read float $y0 Bottom y-coordinate
 * @property-read float $x1 Right x-coordinate
 * @property-read float $y1 Top y-coordinate
 */
readonly class BoundingBox
{
    public function __construct(
        public float $x0,
        public float $y0,
        public float $x1,
        public float $y1,
    ) {
    }

    /**
     * Create BoundingBox from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $x0 = is_numeric($data['x0'] ?? null) ? (float) $data['x0'] : 0.0;
        $y0 = is_numeric($data['y0'] ?? null) ? (float) $data['y0'] : 0.0;
        $x1 = is_numeric($data['x1'] ?? null) ? (float) $data['x1'] : 0.0;
        $y1 = is_numeric($data['y1'] ?? null) ? (float) $data['y1'] : 0.0;

        return new self(
            x0: $x0,
            y0: $y0,
            x1: $x1,
            y1: $y1,
        );
    }
}

/**
 * Metadata for a semantic element.
 *
 * @property-read int|null $pageNumber Page number (1-indexed)
 * @property-read string|null $filename Source filename or document name
 * @property-read BoundingBox|null $coordinates Bounding box coordinates if available
 * @property-read int|null $elementIndex Position index in the element sequence
 * @property-read array<string, string> $additional Additional custom metadata
 */
readonly class ElementMetadata
{
    /**
     * @param array<string, string> $additional
     */
    public function __construct(
        public ?int $pageNumber = null,
        public ?string $filename = null,
        public ?BoundingBox $coordinates = null,
        public ?int $elementIndex = null,
        public array $additional = [],
    ) {
    }

    /**
     * Create ElementMetadata from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $pageNumber = isset($data['page_number']) && is_numeric($data['page_number'])
            ? (int) $data['page_number']
            : null;

        /** @var string|null $filename */
        $filename = $data['filename'] ?? null;

        $coordinates = null;
        if (isset($data['coordinates'])) {
            /** @var array<string, mixed> $coordinatesData */
            $coordinatesData = $data['coordinates'];
            $coordinates = BoundingBox::fromArray($coordinatesData);
        }

        $elementIndex = isset($data['element_index']) && is_numeric($data['element_index'])
            ? (int) $data['element_index']
            : null;

        /** @var array<string, string> $additional */
        $additional = [];
        if (isset($data['additional']) && is_array($data['additional'])) {
            foreach ($data['additional'] as $key => $value) {
                $stringKey = is_string($key) ? $key : (string) $key;
                $stringValue = is_string($value) ? $value : (is_scalar($value) ? (string) $value : '');
                $additional[$stringKey] = $stringValue;
            }
        }

        return new self(
            pageNumber: $pageNumber,
            filename: $filename,
            coordinates: $coordinates,
            elementIndex: $elementIndex,
            additional: $additional,
        );
    }
}

/**
 * Semantic element extracted from document.
 *
 * Represents a logical unit of content with semantic classification,
 * unique identifier, and metadata for tracking origin and position.
 * Compatible with Unstructured element format when output_format='element_based'.
 *
 * @property-read string $elementId Unique element identifier (deterministic hash-based ID)
 * @property-read string $elementType Semantic type classification
 * @property-read string $text Content string
 * @property-read ElementMetadata $metadata Element metadata including page number, coordinates, etc.
 */
readonly class Element
{
    public function __construct(
        public string $elementId,
        public string $elementType,
        public string $text,
        public ElementMetadata $metadata,
    ) {
    }

    /**
     * Create Element from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $elementId */
        $elementId = $data['element_id'] ?? '';

        /** @var string $elementType */
        $elementType = $data['element_type'] ?? '';

        /** @var string $text */
        $text = $data['text'] ?? '';

        /** @var array<string, mixed> $metadataData */
        $metadataData = $data['metadata'] ?? [];

        return new self(
            elementId: $elementId,
            elementType: $elementType,
            text: $text,
            metadata: ElementMetadata::fromArray($metadataData),
        );
    }
}

/**
 * Semantic element type classification.
 *
 * Categorizes text content into semantic units for downstream processing.
 * Supports the element types commonly found in document intelligence.
 */
enum ElementType: string
{
    /** Document title */
    case Title = 'title';
    /** Main narrative text body */
    case NarrativeText = 'narrative_text';
    /** Section heading */
    case Heading = 'heading';
    /** List item (bullet, numbered, etc.) */
    case ListItem = 'list_item';
    /** Table element */
    case Table = 'table';
    /** Image element */
    case Image = 'image';
    /** Page break marker */
    case PageBreak = 'page_break';
    /** Code block */
    case CodeBlock = 'code_block';
    /** Block quote */
    case BlockQuote = 'block_quote';
    /** Footer text */
    case Footer = 'footer';
    /** Header text */
    case Header = 'header';
}

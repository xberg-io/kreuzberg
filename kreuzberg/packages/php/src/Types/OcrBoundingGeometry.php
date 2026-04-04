<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Bounding geometry for OCR elements.
 *
 * Represents the geometric positioning of OCR-extracted text,
 * supporting both rectangular (left/top/width/height) and point-based
 * (polygon points) geometry definitions.
 *
 * @property-read string $type Geometry type ("rect" or "polygon")
 * @property-read float|null $left Left x-coordinate for rectangular geometry
 * @property-read float|null $top Top y-coordinate for rectangular geometry
 * @property-read float|null $width Width for rectangular geometry
 * @property-read float|null $height Height for rectangular geometry
 * @property-read array<array<float>>|null $points Array of [x, y] points for polygon geometry
 */
readonly class OcrBoundingGeometry
{
    /**
     * @param array<array<float>>|null $points
     */
    public function __construct(
        public string $type = 'rect',
        public ?float $left = null,
        public ?float $top = null,
        public ?float $width = null,
        public ?float $height = null,
        public ?array $points = null,
    ) {
    }

    /**
     * Create OcrBoundingGeometry from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $type */
        $type = $data['type'] ?? 'rect';

        /** @var float|null $left */
        $left = isset($data['left']) && is_numeric($data['left']) ? (float) $data['left'] : null;

        /** @var float|null $top */
        $top = isset($data['top']) && is_numeric($data['top']) ? (float) $data['top'] : null;

        /** @var float|null $width */
        $width = isset($data['width']) && is_numeric($data['width']) ? (float) $data['width'] : null;

        /** @var float|null $height */
        $height = isset($data['height']) && is_numeric($data['height']) ? (float) $data['height'] : null;

        /** @var array<array<float>>|null $points */
        $points = null;
        if (isset($data['points']) && is_array($data['points'])) {
            $points = array_map(
                /**
                 * @param mixed $point
                 * @return array<float>
                 */
                static function ($point): array {
                    if (is_array($point)) {
                        return [
                            isset($point[0]) && is_numeric($point[0]) ? (float) $point[0] : 0.0,
                            isset($point[1]) && is_numeric($point[1]) ? (float) $point[1] : 0.0,
                        ];
                    }
                    return [0.0, 0.0];
                },
                $data['points'],
            );
        }

        return new self(
            type: $type,
            left: $left,
            top: $top,
            width: $width,
            height: $height,
            points: $points,
        );
    }
}

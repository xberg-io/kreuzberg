<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Rotation information for OCR-detected text.
 *
 * Represents the rotation angle of text as detected by OCR,
 * including the confidence of the angle detection.
 *
 * @property-read float|null $angleDegrees Rotation angle in degrees (-180 to 180)
 * @property-read float|null $confidence Confidence score for rotation detection (0.0-1.0)
 */
readonly class OcrRotation
{
    public function __construct(
        public ?float $angleDegrees = null,
        public ?float $confidence = null,
    ) {
    }

    /**
     * Create OcrRotation from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var float|null $angleDegrees */
        $angleDegrees = isset($data['angle_degrees']) && is_numeric($data['angle_degrees'])
            ? (float) $data['angle_degrees']
            : null;

        /** @var float|null $confidence */
        $confidence = isset($data['confidence']) && is_numeric($data['confidence'])
            ? (float) $data['confidence']
            : null;

        return new self(
            angleDegrees: $angleDegrees,
            confidence: $confidence,
        );
    }
}

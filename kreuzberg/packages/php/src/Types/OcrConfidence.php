<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Confidence scores for OCR text detection and recognition.
 *
 * Contains separate confidence metrics for different stages of the OCR process:
 * detection (finding text regions) and recognition (reading the text).
 *
 * @property-read float|null $detection Confidence score for text detection (0.0-1.0)
 * @property-read float|null $recognition Confidence score for text recognition (0.0-1.0)
 */
readonly class OcrConfidence
{
    public function __construct(
        public ?float $detection = null,
        public ?float $recognition = null,
    ) {
    }

    /**
     * Create OcrConfidence from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var float|null $detection */
        $detection = isset($data['detection']) && is_numeric($data['detection']) ? (float) $data['detection'] : null;

        /** @var float|null $recognition */
        $recognition = isset($data['recognition']) && is_numeric($data['recognition']) ? (float) $data['recognition'] : null;

        return new self(
            detection: $detection,
            recognition: $recognition,
        );
    }
}

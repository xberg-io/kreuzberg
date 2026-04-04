<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Image artifact extracted from a document page.
 *
 * @property-read string $data Image data (bytes)
 * @property-read string $format Image format (e.g., "png", "jpeg")
 * @property-read int $imageIndex Image index within document
 * @property-read int|null $pageNumber Page number where image was found
 * @property-read int|null $width Image width in pixels
 * @property-read int|null $height Image height in pixels
 * @property-read string|null $colorspace Image colorspace
 * @property-read int|null $bitsPerComponent Bits per color component
 * @property-read bool $isMask Whether image is a mask
 * @property-read string|null $description Image description/alt text
 * @property-read ExtractionResult|null $ocrResult OCR result if OCR was performed on this image
 * @property-read BoundingBox|null $boundingBox Bounding box coordinates if available
 */
readonly class ExtractedImage
{
    public function __construct(
        public string $data,
        public string $format,
        public int $imageIndex,
        public ?int $pageNumber = null,
        public ?int $width = null,
        public ?int $height = null,
        public ?string $colorspace = null,
        public ?int $bitsPerComponent = null,
        public bool $isMask = false,
        public ?string $description = null,
        public ?ExtractionResult $ocrResult = null,
        public ?BoundingBox $boundingBox = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $imageData */
        $imageData = $data['data'] ?? '';

        /** @var string $format */
        $format = $data['format'] ?? '';

        /** @var int $imageIndex */
        $imageIndex = $data['image_index'] ?? 0;

        /** @var int|null $pageNumber */
        $pageNumber = $data['page_number'] ?? null;

        /** @var int|null $width */
        $width = $data['width'] ?? null;

        /** @var int|null $height */
        $height = $data['height'] ?? null;

        /** @var string|null $colorspace */
        $colorspace = $data['colorspace'] ?? null;

        /** @var int|null $bitsPerComponent */
        $bitsPerComponent = $data['bits_per_component'] ?? null;

        /** @var bool $isMask */
        $isMask = $data['is_mask'] ?? false;

        /** @var string|null $description */
        $description = $data['description'] ?? null;

        $ocrResult = null;
        if (isset($data['ocr_result'])) {
            /** @var array<string, mixed> $ocrResultData */
            $ocrResultData = $data['ocr_result'];
            $ocrResult = ExtractionResult::fromArray($ocrResultData);
        }

        $boundingBox = null;
        if (isset($data['bounding_box'])) {
            /** @var array<string, mixed> $boundingBoxData */
            $boundingBoxData = $data['bounding_box'];
            $boundingBox = BoundingBox::fromArray($boundingBoxData);
        }

        return new self(
            data: $imageData,
            format: $format,
            imageIndex: $imageIndex,
            pageNumber: $pageNumber,
            width: $width,
            height: $height,
            colorspace: $colorspace,
            bitsPerComponent: $bitsPerComponent,
            isMask: $isMask,
            description: $description,
            ocrResult: $ocrResult,
            boundingBox: $boundingBox,
        );
    }
}

<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

class ImageMetadata
{
    public string $src;
    public ?string $alt = null;
    public ?string $title = null;
    /** @var int[]|null */
    public ?array $dimensions = null;
    public string $imageType = 'external';
    /** @var array<string, string> */
    public array $attributes = [];

    /**
     * @param int[]|null $dimensions
     * @param array<string, string> $attributes
     */
    public function __construct(
        string $src,
        string $imageType = 'external',
        ?string $alt = null,
        ?string $title = null,
        ?array $dimensions = null,
        array $attributes = [],
    ) {
        $this->src = $src;
        $this->imageType = $imageType;
        $this->alt = $alt;
        $this->title = $title;
        $this->dimensions = $dimensions;
        $this->attributes = $attributes;
    }
}

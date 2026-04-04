<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

class LinkMetadata
{
    public string $href;
    public string $text;
    public ?string $title = null;
    public string $linkType = 'other';
    /** @var string[] */
    public array $rel = [];
    /** @var array<string, string> */
    public array $attributes = [];

    /**
     * @param string[] $rel
     * @param array<string, string> $attributes
     */
    public function __construct(
        string $href,
        string $text,
        string $linkType = 'other',
        ?string $title = null,
        array $rel = [],
        array $attributes = [],
    ) {
        $this->href = $href;
        $this->text = $text;
        $this->linkType = $linkType;
        $this->title = $title;
        $this->rel = $rel;
        $this->attributes = $attributes;
    }
}

<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

class HeaderMetadata
{
    public int $level;
    public string $text;
    public ?string $id = null;
    public int $depth;
    public int $htmlOffset;

    public function __construct(
        int $level,
        string $text,
        int $depth,
        int $htmlOffset,
        ?string $id = null,
    ) {
        $this->level = $level;
        $this->text = $text;
        $this->depth = $depth;
        $this->htmlOffset = $htmlOffset;
        $this->id = $id;
    }
}

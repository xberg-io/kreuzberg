<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

class StructuredData
{
    public string $dataType;
    public string $rawJson;
    public ?string $schemaType = null;

    public function __construct(
        string $dataType,
        string $rawJson,
        ?string $schemaType = null,
    ) {
        $this->dataType = $dataType;
        $this->rawJson = $rawJson;
        $this->schemaType = $schemaType;
    }
}

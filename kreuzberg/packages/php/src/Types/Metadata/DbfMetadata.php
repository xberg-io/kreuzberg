<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * dBASE (DBF) file metadata.
 *
 * Contains record count, field count, and field definitions.
 */
readonly class DbfMetadata
{
    /**
     * @param DbfFieldInfo[] $fields
     */
    public function __construct(
        public int $recordCount = 0,
        public int $fieldCount = 0,
        public array $fields = [],
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawRecordCount = $data['record_count'] ?? 0;
        $recordCount = is_int($rawRecordCount) ? $rawRecordCount : (is_numeric($rawRecordCount) ? (int) $rawRecordCount : 0);

        $rawFieldCount = $data['field_count'] ?? 0;
        $fieldCount = is_int($rawFieldCount) ? $rawFieldCount : (is_numeric($rawFieldCount) ? (int) $rawFieldCount : 0);

        /** @var DbfFieldInfo[] $fields */
        $fields = [];
        $rawFields = $data['fields'] ?? null;
        if (is_array($rawFields)) {
            foreach ($rawFields as $fieldData) {
                if (is_array($fieldData)) {
                    /** @var array<string, mixed> $typedFieldData */
                    $typedFieldData = $fieldData;
                    $fields[] = DbfFieldInfo::fromArray($typedFieldData);
                }
            }
        }

        return new self(
            recordCount: $recordCount,
            fieldCount: $fieldCount,
            fields: $fields,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [
            'record_count' => $this->recordCount,
            'field_count' => $this->fieldCount,
        ];

        if ($this->fields !== []) {
            $result['fields'] = array_map(
                fn (DbfFieldInfo $f) => $f->toArray(),
                $this->fields,
            );
        }

        return $result;
    }
}

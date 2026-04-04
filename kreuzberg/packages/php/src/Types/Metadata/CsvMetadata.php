<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * CSV/TSV file metadata.
 *
 * Contains information about CSV file structure including row/column counts,
 * delimiter, and column type information.
 */
readonly class CsvMetadata
{
    /**
     * @param string[]|null $columnTypes
     */
    public function __construct(
        public int $rowCount = 0,
        public int $columnCount = 0,
        public ?string $delimiter = null,
        public bool $hasHeader = false,
        public ?array $columnTypes = null,
    ) {
    }

    /**
     * Create CsvMetadata from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawRowCount = $data['row_count'] ?? 0;
        $rowCount = is_int($rawRowCount) ? $rawRowCount : (is_numeric($rawRowCount) ? (int) $rawRowCount : 0);

        $rawColumnCount = $data['column_count'] ?? 0;
        $columnCount = is_int($rawColumnCount) ? $rawColumnCount : (is_numeric($rawColumnCount) ? (int) $rawColumnCount : 0);

        /** @var string|null $delimiter */
        $delimiter = $data['delimiter'] ?? null;

        /** @var bool $hasHeader */
        $hasHeader = (bool) ($data['has_header'] ?? false);

        /** @var string[]|null $columnTypes */
        $columnTypes = $data['column_types'] ?? null;
        if (!is_array($columnTypes)) {
            $columnTypes = null;
        }

        return new self(
            rowCount: $rowCount,
            columnCount: $columnCount,
            delimiter: $delimiter,
            hasHeader: $hasHeader,
            columnTypes: $columnTypes,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [
            'row_count' => $this->rowCount,
            'column_count' => $this->columnCount,
            'has_header' => $this->hasHeader,
        ];

        if ($this->delimiter !== null) {
            $result['delimiter'] = $this->delimiter;
        }

        if ($this->columnTypes !== null) {
            $result['column_types'] = $this->columnTypes;
        }

        return $result;
    }
}

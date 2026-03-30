<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Byte and line/column span for a code element.
 *
 * @property-read int $startByte Starting byte offset
 * @property-read int $endByte Ending byte offset
 * @property-read int $startLine Starting line number
 * @property-read int $startColumn Starting column number
 * @property-read int $endLine Ending line number
 * @property-read int $endColumn Ending column number
 */
readonly class CodeSpan
{
    public function __construct(
        public int $startByte,
        public int $endByte,
        public int $startLine,
        public int $startColumn,
        public int $endLine,
        public int $endColumn,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int $startByte */
        $startByte = $data['start_byte'] ?? 0;

        /** @var int $endByte */
        $endByte = $data['end_byte'] ?? 0;

        /** @var int $startLine */
        $startLine = $data['start_line'] ?? 0;

        /** @var int $startColumn */
        $startColumn = $data['start_column'] ?? 0;

        /** @var int $endLine */
        $endLine = $data['end_line'] ?? 0;

        /** @var int $endColumn */
        $endColumn = $data['end_column'] ?? 0;

        return new self(
            startByte: $startByte,
            endByte: $endByte,
            startLine: $startLine,
            startColumn: $startColumn,
            endLine: $endLine,
            endColumn: $endColumn,
        );
    }
}

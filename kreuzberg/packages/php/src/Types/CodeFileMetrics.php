<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * File-level code metrics from tree-sitter analysis.
 *
 * @property-read int $totalLines Total number of lines
 * @property-read int $codeLines Number of code lines
 * @property-read int $commentLines Number of comment lines
 * @property-read int $blankLines Number of blank lines
 * @property-read int $totalBytes Total byte size
 * @property-read int $nodeCount Number of AST nodes
 * @property-read int $errorCount Number of parse errors
 * @property-read int $maxDepth Maximum AST depth
 */
readonly class CodeFileMetrics
{
    public function __construct(
        public int $totalLines,
        public int $codeLines,
        public int $commentLines,
        public int $blankLines,
        public int $totalBytes,
        public int $nodeCount,
        public int $errorCount,
        public int $maxDepth,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int $totalLines */
        $totalLines = $data['total_lines'] ?? 0;

        /** @var int $codeLines */
        $codeLines = $data['code_lines'] ?? 0;

        /** @var int $commentLines */
        $commentLines = $data['comment_lines'] ?? 0;

        /** @var int $blankLines */
        $blankLines = $data['blank_lines'] ?? 0;

        /** @var int $totalBytes */
        $totalBytes = $data['total_bytes'] ?? 0;

        /** @var int $nodeCount */
        $nodeCount = $data['node_count'] ?? 0;

        /** @var int $errorCount */
        $errorCount = $data['error_count'] ?? 0;

        /** @var int $maxDepth */
        $maxDepth = $data['max_depth'] ?? 0;

        return new self(
            totalLines: $totalLines,
            codeLines: $codeLines,
            commentLines: $commentLines,
            blankLines: $blankLines,
            totalBytes: $totalBytes,
            nodeCount: $nodeCount,
            errorCount: $errorCount,
            maxDepth: $maxDepth,
        );
    }
}

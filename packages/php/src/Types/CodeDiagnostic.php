<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Parse diagnostic (error or warning from tree-sitter).
 *
 * @property-read string $message Diagnostic message
 * @property-read string $severity Severity level (e.g. 'error', 'warning')
 * @property-read CodeSpan $span Source span
 */
readonly class CodeDiagnostic
{
    public function __construct(
        public string $message,
        public string $severity,
        public CodeSpan $span,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $message */
        $message = $data['message'] ?? '';

        /** @var string $severity */
        $severity = $data['severity'] ?? '';

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        return new self(
            message: $message,
            severity: $severity,
            span: CodeSpan::fromArray($spanData),
        );
    }
}

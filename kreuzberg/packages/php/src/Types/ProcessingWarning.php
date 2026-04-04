<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Warning generated during document processing.
 *
 * @property-read string $source Source component that generated the warning
 * @property-read string $message Warning message
 */
readonly class ProcessingWarning
{
    public function __construct(
        public string $source,
        public string $message,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $source */
        $source = $data['source'] ?? '';

        /** @var string $message */
        $message = $data['message'] ?? '';

        return new self(
            source: $source,
            message: $message,
        );
    }
}

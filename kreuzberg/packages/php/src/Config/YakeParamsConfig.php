<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Algorithm-specific parameters for the YAKE keyword extraction algorithm.
 */
readonly class YakeParamsConfig
{
    public function __construct(
        /**
         * Window size for co-occurrence statistics.
         *
         * @var int|null
         * @default null (use algorithm default)
         */
        public ?int $windowSize = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $windowSize = $data['window_size'] ?? null;

        return new self(
            windowSize: is_int($windowSize) ? $windowSize : null,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'window_size' => $this->windowSize,
        ], static fn ($value): bool => $value !== null);
    }
}

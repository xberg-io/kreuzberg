<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Concurrency configuration for thread pool management.
 *
 * Controls the maximum number of threads used for parallel processing
 * during document extraction.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\ConcurrencyConfig;
 *
 * $concurrency = new ConcurrencyConfig(
 *     maxThreads: 4,
 * );
 * ```
 */
readonly class ConcurrencyConfig
{
    /**
     * @param int|null $maxThreads Maximum number of threads for parallel processing.
     *                             Default is null (use Rust default).
     */
    public function __construct(
        public ?int $maxThreads = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $maxThreads = $data['max_threads'] ?? null;

        return new self(
            maxThreads: is_int($maxThreads) ? $maxThreads : null,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [];

        if ($this->maxThreads !== null) {
            $result['max_threads'] = $this->maxThreads;
        }

        return $result;
    }
}

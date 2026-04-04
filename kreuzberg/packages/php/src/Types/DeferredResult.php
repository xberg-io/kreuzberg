<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Deferred result from an async extraction operation.
 *
 * Returned immediately from async extraction functions. The actual extraction
 * runs on a background Tokio worker thread. Use isReady(), getResult(),
 * tryGetResult(), or wait() to retrieve the result.
 *
 * This class is provided by the kreuzberg-php extension. This PHP file
 * serves as a type stub for IDE support and documentation.
 *
 * @example
 * ```php
 * $deferred = kreuzberg_extract_file_async('document.pdf');
 *
 * // Non-blocking check
 * if ($deferred->isReady()) {
 *     $result = $deferred->getResult();
 * }
 *
 * // Blocking wait
 * $result = $deferred->getResult();
 *
 * // Wait with timeout (milliseconds)
 * $result = $deferred->wait(5000);
 * ```
 */
class DeferredResult
{
    /**
     * Check if the result is ready (non-blocking).
     *
     * @return bool True if the result is available, false if still processing
     */
    public function isReady(): bool
    {
        throw new \BadMethodCallException('Provided by native extension');
    }

    /**
     * Try to get the result without blocking.
     *
     * @return ExtractionResult|null The result if ready, or null if still processing
     *
     * @throws \Exception If the extraction failed
     */
    public function tryGetResult(): ?ExtractionResult
    {
        throw new \BadMethodCallException('Provided by native extension');
    }

    /**
     * Get the result, blocking until it's ready.
     *
     * @return ExtractionResult The extraction result
     *
     * @throws \Exception If the extraction failed
     */
    public function getResult(): ExtractionResult
    {
        throw new \BadMethodCallException('Provided by native extension');
    }

    /**
     * Get batch results, blocking until ready.
     *
     * @return array<ExtractionResult> Array of extraction results
     *
     * @throws \Exception If the extraction failed or this is not a batch operation
     */
    public function getResults(): array
    {
        throw new \BadMethodCallException('Provided by native extension');
    }

    /**
     * Wait for the result with a timeout.
     *
     * @param int $timeoutMs Maximum time to wait in milliseconds
     *
     * @return ExtractionResult|null The result if ready within timeout, or null if timed out
     *
     * @throws \Exception If the extraction failed
     */
    public function wait(int $timeoutMs): ?ExtractionResult
    {
        throw new \BadMethodCallException('Provided by native extension');
    }

    /**
     * Wait for batch results with a timeout.
     *
     * @param int $timeoutMs Maximum time to wait in milliseconds
     *
     * @return array<ExtractionResult>|null Array of results if ready, or null if timed out
     *
     * @throws \Exception If the extraction failed
     */
    public function waitBatch(int $timeoutMs): ?array
    {
        throw new \BadMethodCallException('Provided by native extension');
    }
}

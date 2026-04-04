<?php

declare(strict_types=1);

namespace Kreuzberg\Async;

use Kreuzberg\Types\DeferredResult;
use Kreuzberg\Types\ExtractionResult;

/**
 * Bridge for integrating DeferredResult with Amp framework.
 *
 * Requires amphp/amp ^3.0. Uses Fiber-based suspension for non-blocking polling.
 *
 * @example
 * ```php
 * use Kreuzberg\Async\AmpBridge;
 * use function Amp\async;
 * use function Amp\Future\await;
 *
 * $deferred = kreuzberg_extract_file_async('document.pdf');
 * $future = AmpBridge::toFuture($deferred);
 * $result = $future->await();
 * echo $result->content;
 *
 * // Parallel extraction with Amp
 * $futures = [];
 * foreach ($files as $file) {
 *     $d = kreuzberg_extract_file_async($file);
 *     $futures[] = AmpBridge::toFuture($d);
 * }
 * $results = await($futures);
 * ```
 */
final class AmpBridge
{
    /**
     * Convert a DeferredResult to an Amp Future.
     *
     * Polls isReady() with adaptive backoff using Amp's delay() for non-blocking suspension.
     *
     * @return \Amp\Future<ExtractionResult>
     * @throws \RuntimeException If amphp/amp is not installed
     */
    public static function toFuture(DeferredResult $deferred): mixed
    {
        if (!class_exists(\Amp\Future::class)) {
            throw new \RuntimeException(
                'amphp/amp ^3.0 is required for AmpBridge. Install it with: composer require amphp/amp:^3.0',
            );
        }

        return \Amp\async(static function () use ($deferred): ExtractionResult {
            $backoffMs = 1;
            $maxBackoffMs = 50;

            while (!$deferred->isReady()) {
                \Amp\delay($backoffMs / 1000.0);
                $backoffMs = min($backoffMs * 2, $maxBackoffMs);
            }

            return $deferred->getResult();
        });
    }

    /**
     * Convert a batch DeferredResult to an Amp Future.
     *
     * @return \Amp\Future<array<ExtractionResult>>
     * @throws \RuntimeException If amphp/amp is not installed
     */
    public static function toBatchFuture(DeferredResult $deferred): mixed
    {
        if (!class_exists(\Amp\Future::class)) {
            throw new \RuntimeException(
                'amphp/amp ^3.0 is required for AmpBridge. Install it with: composer require amphp/amp:^3.0',
            );
        }

        return \Amp\async(static function () use ($deferred): array {
            $backoffMs = 1;
            $maxBackoffMs = 50;

            while (!$deferred->isReady()) {
                \Amp\delay($backoffMs / 1000.0);
                $backoffMs = min($backoffMs * 2, $maxBackoffMs);
            }

            return $deferred->getResults();
        });
    }
}

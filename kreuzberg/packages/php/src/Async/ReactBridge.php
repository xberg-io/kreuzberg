<?php

declare(strict_types=1);

namespace Kreuzberg\Async;

use Kreuzberg\Types\DeferredResult;
use Kreuzberg\Types\ExtractionResult;

/**
 * Bridge for integrating DeferredResult with ReactPHP.
 *
 * Requires react/promise ^3.0 and react/event-loop ^1.0.
 * Uses periodic timers to poll the DeferredResult without blocking the event loop.
 *
 * @example
 * ```php
 * use Kreuzberg\Async\ReactBridge;
 * use React\EventLoop\Loop;
 *
 * $deferred = kreuzberg_extract_file_async('document.pdf');
 * $promise = ReactBridge::toPromise($deferred);
 *
 * $promise->then(function (ExtractionResult $result) {
 *     echo $result->content;
 * });
 *
 * Loop::run();
 * ```
 */
final class ReactBridge
{
    /**
     * Convert a DeferredResult to a ReactPHP Promise.
     *
     * @return \React\Promise\PromiseInterface<ExtractionResult>
     * @throws \RuntimeException If react/promise is not installed
     */
    public static function toPromise(DeferredResult $deferred): mixed
    {
        if (!class_exists(\React\Promise\Deferred::class)) {
            throw new \RuntimeException(
                'react/promise ^3.0 is required for ReactBridge. Install it with: composer require react/promise:^3.0',
            );
        }

        $reactDeferred = new \React\Promise\Deferred();
        $pollIntervalSeconds = 0.001; // 1ms initial interval
        $maxPollInterval = 0.05; // 50ms max interval

        $timer = null;
        $currentInterval = $pollIntervalSeconds;

        $poll = static function () use ($deferred, $reactDeferred, &$timer, &$currentInterval, $maxPollInterval): void {
            if ($deferred->isReady()) {
                if ($timer !== null) {
                    \React\EventLoop\Loop::cancelTimer($timer);
                }
                try {
                    $result = $deferred->getResult();
                    $reactDeferred->resolve($result);
                } catch (\Exception $e) {
                    $reactDeferred->reject($e);
                }
            } else {
                // Adaptive backoff: increase interval
                $currentInterval = min($currentInterval * 2, $maxPollInterval);
                if ($timer !== null) {
                    \React\EventLoop\Loop::cancelTimer($timer);
                }
                $timer = \React\EventLoop\Loop::addTimer($currentInterval, static function () use ($deferred, $reactDeferred, &$timer, &$currentInterval, $maxPollInterval): void {
                    // Re-check inside timer
                    if ($deferred->isReady()) {
                        try {
                            $result = $deferred->getResult();
                            $reactDeferred->resolve($result);
                        } catch (\Exception $e) {
                            $reactDeferred->reject($e);
                        }
                    } else {
                        $currentInterval = min($currentInterval * 2, $maxPollInterval);
                        // Schedule next poll
                        self::schedulePoll($deferred, $reactDeferred, $timer, $currentInterval, $maxPollInterval);
                    }
                });
            }
        };

        // Start first poll
        $timer = \React\EventLoop\Loop::addTimer($pollIntervalSeconds, $poll);

        return $reactDeferred->promise();
    }

    /**
     * Schedule the next poll timer.
     */
    private static function schedulePoll(
        DeferredResult $deferred,
        \React\Promise\Deferred $reactDeferred,
        mixed &$timer,
        float &$currentInterval,
        float $maxPollInterval,
    ): void {
        $timer = \React\EventLoop\Loop::addTimer($currentInterval, static function () use ($deferred, $reactDeferred, &$timer, &$currentInterval, $maxPollInterval): void {
            if ($deferred->isReady()) {
                try {
                    $result = $deferred->getResult();
                    $reactDeferred->resolve($result);
                } catch (\Exception $e) {
                    $reactDeferred->reject($e);
                }
            } else {
                $currentInterval = min($currentInterval * 2, $maxPollInterval);
                self::schedulePoll($deferred, $reactDeferred, $timer, $currentInterval, $maxPollInterval);
            }
        });
    }
}

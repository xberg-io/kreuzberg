<?php

declare(strict_types=1);

namespace Kreuzberg\Exceptions;

/**
 * Context information from a Rust panic.
 *
 * This class captures information about where and why a panic occurred
 * in the underlying Rust code, providing debugging information.
 */
class PanicContext
{
    /**
     * Create a new PanicContext.
     *
     * @param string|null $file Source file where the panic occurred
     * @param int|null $line Line number where the panic occurred
     * @param string|null $function Function name where the panic occurred
     * @param string|null $message Panic message
     * @param int|null $timestampSecs Unix timestamp when the panic occurred
     */
    public function __construct(
        public readonly ?string $file,
        public readonly ?int $line,
        public readonly ?string $function,
        public readonly ?string $message,
        public readonly ?int $timestampSecs,
    ) {
    }

    /**
     * Create a PanicContext from a JSON string.
     *
     * @param string $json JSON string containing panic context data
     * @return self|null Returns null if JSON is invalid or empty
     */
    public static function fromJson(string $json): ?self
    {
        $data = json_decode($json, true);
        if (!is_array($data)) {
            return null;
        }
        /** @var array<string, mixed> $data */
        return self::fromArray($data);
    }

    /**
     * Create a PanicContext from an associative array.
     *
     * @param array<string, mixed> $data Array containing panic context data
     * @return self
     */
    public static function fromArray(array $data): self
    {
        return new self(
            file: isset($data['file']) && is_string($data['file']) ? $data['file'] : null,
            line: isset($data['line']) && is_int($data['line']) ? $data['line'] : null,
            function: isset($data['function']) && is_string($data['function']) ? $data['function'] : null,
            message: isset($data['message']) && is_string($data['message']) ? $data['message'] : null,
            timestampSecs: isset($data['timestamp_secs']) && is_int($data['timestamp_secs']) ? $data['timestamp_secs'] : null,
        );
    }
}

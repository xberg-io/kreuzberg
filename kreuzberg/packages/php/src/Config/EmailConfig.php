<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Email extraction configuration.
 *
 * Configures settings for email file extraction, including fallback
 * code page handling for MSG format bodies.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\EmailConfig;
 *
 * $email = new EmailConfig(
 *     msgFallbackCodepage: 1252,
 * );
 * ```
 */
readonly class EmailConfig
{
    /**
     * @param int|null $msgFallbackCodepage Fallback code page for MSG email body decoding.
     *                                      Default is null (use Rust default).
     */
    public function __construct(
        public ?int $msgFallbackCodepage = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $msgFallbackCodepage = $data['msg_fallback_codepage'] ?? null;

        return new self(
            msgFallbackCodepage: is_int($msgFallbackCodepage) ? $msgFallbackCodepage : null,
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

        if ($this->msgFallbackCodepage !== null) {
            $result['msg_fallback_codepage'] = $this->msgFallbackCodepage;
        }

        return $result;
    }
}

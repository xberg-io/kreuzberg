<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * Outlook PST archive metadata.
 *
 * Contains message count from PST archive files.
 */
readonly class PstMetadata
{
    public function __construct(
        public int $messageCount = 0,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawCount = $data['message_count'] ?? 0;
        $messageCount = is_int($rawCount) ? $rawCount : (is_numeric($rawCount) ? (int) $rawCount : 0);

        return new self(
            messageCount: $messageCount,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'message_count' => $this->messageCount,
        ];
    }
}

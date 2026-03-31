<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Comment information.
 *
 * @property-read string $text Comment text
 * @property-read string $kind Comment kind (e.g. 'line', 'block')
 * @property-read CodeSpan $span Source span
 */
readonly class CodeCommentInfo
{
    public function __construct(
        public string $text,
        public string $kind,
        public CodeSpan $span,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var string $text */
        $text = $data['text'] ?? '';

        /** @var string $kind */
        $kind = $data['kind'] ?? '';

        /** @var array<string, mixed> $spanData */
        $spanData = $data['span'] ?? [];

        return new self(
            text: $text,
            kind: $kind,
            span: CodeSpan::fromArray($spanData),
        );
    }
}

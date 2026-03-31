<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * JATS contributor with role.
 */
readonly class ContributorRole
{
    public function __construct(
        public string $name,
        public ?string $role = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawName = $data['name'] ?? '';
        $rawRole = $data['role'] ?? null;

        return new self(
            name: is_string($rawName) ? $rawName : '',
            role: is_string($rawRole) ? $rawRole : null,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [
            'name' => $this->name,
        ];

        if ($this->role !== null) {
            $result['role'] = $this->role;
        }

        return $result;
    }
}

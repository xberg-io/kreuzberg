<?php

declare(strict_types=1);

namespace Kreuzberg\Types\Metadata;

/**
 * JATS (Journal Article Tag Suite) metadata.
 *
 * Contains copyright, license, publication history dates,
 * and contributor role information.
 */
readonly class JatsMetadata
{
    /**
     * @param array<string, string> $historyDates
     * @param ContributorRole[] $contributorRoles
     */
    public function __construct(
        public ?string $copyright = null,
        public ?string $license = null,
        public array $historyDates = [],
        public array $contributorRoles = [],
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawCopyright = $data['copyright'] ?? null;
        $copyright = is_string($rawCopyright) ? $rawCopyright : null;

        $rawLicense = $data['license'] ?? null;
        $license = is_string($rawLicense) ? $rawLicense : null;

        /** @var array<string, string> $historyDates */
        $historyDates = $data['history_dates'] ?? [];
        if (!is_array($historyDates)) {
            $historyDates = [];
        }

        /** @var ContributorRole[] $contributorRoles */
        $contributorRoles = [];
        $rawRoles = $data['contributor_roles'] ?? null;
        if (is_array($rawRoles)) {
            foreach ($rawRoles as $roleData) {
                if (is_array($roleData)) {
                    /** @var array<string, mixed> $typedRoleData */
                    $typedRoleData = $roleData;
                    $contributorRoles[] = ContributorRole::fromArray($typedRoleData);
                }
            }
        }

        return new self(
            copyright: $copyright,
            license: $license,
            historyDates: $historyDates,
            contributorRoles: $contributorRoles,
        );
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [];

        if ($this->copyright !== null) {
            $result['copyright'] = $this->copyright;
        }

        if ($this->license !== null) {
            $result['license'] = $this->license;
        }

        if ($this->historyDates !== []) {
            $result['history_dates'] = $this->historyDates;
        }

        if ($this->contributorRoles !== []) {
            $result['contributor_roles'] = array_map(
                fn (ContributorRole $c) => $c->toArray(),
                $this->contributorRoles,
            );
        }

        return $result;
    }
}

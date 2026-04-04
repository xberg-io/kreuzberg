<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Tree-sitter configuration for code parsing.
 *
 * Configures tree-sitter grammar management and code extraction behavior,
 * including cache directory, language selection, language groups,
 * and process-level extraction options.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\TreeSitterConfig;
 * use Kreuzberg\Config\TreeSitterProcessConfig;
 *
 * $treeSitter = new TreeSitterConfig(
 *     cacheDir: '/tmp/grammars',
 *     languages: ['python', 'javascript'],
 *     process: new TreeSitterProcessConfig(
 *         structure: true,
 *         comments: true,
 *     ),
 * );
 * ```
 */
readonly class TreeSitterConfig
{
    /**
     * @param bool|null $enabled Enable code intelligence processing. Default true.
     * @param string|null $cacheDir Custom cache directory for downloaded grammars.
     *                              Default is null (use engine default).
     * @param string[]|null $languages Languages to pre-download on init.
     *                                 Default is null.
     * @param string[]|null $groups Language groups to pre-download.
     *                              Default is null.
     * @param TreeSitterProcessConfig|null $process Tree-sitter process configuration.
     *                                              Default is null (use engine defaults).
     */
    public function __construct(
        public ?bool $enabled = null,
        public ?string $cacheDir = null,
        public ?array $languages = null,
        public ?array $groups = null,
        public ?TreeSitterProcessConfig $process = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var bool|null $enabled */
        $enabled = isset($data['enabled']) && is_bool($data['enabled']) ? $data['enabled'] : null;

        /** @var string|null $cacheDir */
        $cacheDir = $data['cache_dir'] ?? null;
        if ($cacheDir !== null && !is_string($cacheDir)) {
            /** @var string $cacheDir */
            $cacheDir = (string) $cacheDir;
        }

        /** @var string[]|null $languages */
        $languages = null;
        if (isset($data['languages']) && is_array($data['languages'])) {
            $languages = array_values(array_filter(
                $data['languages'],
                static fn (mixed $v): bool => is_string($v),
            ));
        }

        /** @var string[]|null $groups */
        $groups = null;
        if (isset($data['groups']) && is_array($data['groups'])) {
            $groups = array_values(array_filter(
                $data['groups'],
                static fn (mixed $v): bool => is_string($v),
            ));
        }

        $process = null;
        if (isset($data['process']) && is_array($data['process'])) {
            /** @var array<string, mixed> $processData */
            $processData = $data['process'];
            $process = TreeSitterProcessConfig::fromArray($processData);
        }

        return new self(
            enabled: $enabled,
            cacheDir: $cacheDir,
            languages: $languages,
            groups: $groups,
            process: $process,
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

        if ($this->enabled !== null) {
            $result['enabled'] = $this->enabled;
        }
        if ($this->cacheDir !== null) {
            $result['cache_dir'] = $this->cacheDir;
        }
        if ($this->languages !== null) {
            $result['languages'] = $this->languages;
        }
        if ($this->groups !== null) {
            $result['groups'] = $this->groups;
        }
        if ($this->process !== null) {
            $result['process'] = $this->process->toArray();
        }

        return $result;
    }
}

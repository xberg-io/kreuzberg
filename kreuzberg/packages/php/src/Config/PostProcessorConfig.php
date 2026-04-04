<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Post-processor configuration.
 */
readonly class PostProcessorConfig
{
    public function __construct(
        /**
         * Enable post-processing of extracted content.
         *
         * When enabled, extracted text and other content are passed through
         * post-processing pipeline(s) for additional refinement, cleaning,
         * or transformation before being returned to the user.
         *
         * @var bool
         * @default false
         */
        public bool $enabled = false,

        /**
         * Name of the post-processor to apply.
         *
         * Specifies which post-processing algorithm or pipeline to use.
         * The name refers to a registered post-processor implementation.
         *
         * Common values:
         * - 'cleanup': Basic cleanup (whitespace normalization, etc.)
         * - 'normalize': Text normalization and standardization
         * - 'grammar_fix': Basic grammar and spelling correction
         * - 'custom_pipeline': Custom user-defined pipeline
         * - null: No specific processor selected
         *
         * @var string|null
         * @default null
         */
        public ?string $name = null,

        /**
         * Configuration for the post-processor.
         *
         * Post-processor-specific settings passed to the selected processor.
         * Structure depends on the processor implementation. Typically an
         * associative array with processor-specific options.
         *
         * @example
         * ```php
         * $config = [
         *     'enabled' => true,
         *     'name' => 'cleanup',
         *     'config' => [
         *         'normalize_whitespace' => true,
         *         'remove_control_chars' => true,
         *     ]
         * ]
         * ```
         *
         * @var mixed
         * @default null
         */
        public mixed $config = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var bool $enabled */
        $enabled = $data['enabled'] ?? false;
        if (!is_bool($enabled)) {
            /** @var bool $enabled */
            $enabled = (bool) $enabled;
        }

        /** @var string|null $name */
        $name = $data['name'] ?? null;
        if ($name !== null && !is_string($name)) {
            /** @var string $name */
            $name = (string) $name;
        }

        return new self(
            enabled: $enabled,
            name: $name,
            config: $data['config'] ?? null,
        );
    }

    /**
     * Create configuration from JSON string.
     */
    public static function fromJson(string $json): self
    {
        $data = json_decode($json, true);
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \InvalidArgumentException('Invalid JSON: ' . json_last_error_msg());
        }
        if (!is_array($data)) {
            throw new \InvalidArgumentException('JSON must decode to an object/array');
        }
        /** @var array<string, mixed> $data */
        return self::fromArray($data);
    }

    /**
     * Create configuration from JSON file.
     */
    public static function fromFile(string $path): self
    {
        if (!file_exists($path)) {
            throw new \InvalidArgumentException("File not found: {$path}");
        }
        $contents = file_get_contents($path);
        if ($contents === false) {
            throw new \InvalidArgumentException("Unable to read file: {$path}");
        }
        return self::fromJson($contents);
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $array = [
            'enabled' => $this->enabled,
        ];

        if ($this->name !== null) {
            $array['name'] = $this->name;
        }

        if ($this->config !== null) {
            $array['config'] = $this->config;
        }

        return $array;
    }

    /**
     * Convert configuration to JSON string.
     */
    public function toJson(): string
    {
        $json = json_encode($this->toArray(), JSON_PRETTY_PRINT);
        if ($json === false) {
            throw new \RuntimeException('Failed to encode configuration to JSON');
        }
        return $json;
    }
}

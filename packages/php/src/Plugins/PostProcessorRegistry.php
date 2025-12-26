<?php

declare(strict_types=1);

namespace Kreuzberg\Plugins;

use Kreuzberg\Types\ExtractionResult;

/**
 * Registry for managing post-processor plugins.
 *
 * This class provides a high-level PHP API for registering and managing
 * post-processor plugins. It wraps the lower-level extension functions
 * with type-safe, object-oriented methods.
 *
 * @example Register a post-processor
 * ```php
 * $registry = PostProcessorRegistry::getInstance();
 *
 * $registry->register('word_count', function(ExtractionResult $result): ExtractionResult {
 *     $wordCount = str_word_count($result->content);
 *     $result->metadata->additional['word_count'] = $wordCount;
 *     return $result;
 * });
 * ```
 *
 * @example Use a class-based post-processor
 * ```php
 * class EntityExtractor implements PostProcessorInterface
 * {
 *     public function process(ExtractionResult $result): ExtractionResult
 *     {
 *         // Extract entities from text
 *         $entities = $this->extractEntities($result->content);
 *         $result->metadata->additional['entities'] = $entities;
 *         return $result;
 *     }
 *
 *     private function extractEntities(string $text): array
 *     {
 *         // Entity extraction logic
 *         return ['PERSON' => ['John Doe'], 'ORG' => ['Microsoft']];
 *     }
 * }
 *
 * $registry = PostProcessorRegistry::getInstance();
 * $processor = new EntityExtractor();
 * $registry->registerInstance('entities', $processor);
 * ```
 */
final class PostProcessorRegistry
{
    private static ?self $instance = null;

    /**
     * Private constructor to enforce singleton pattern.
     */
    private function __construct()
    {
    }

    /**
     * Get the singleton registry instance.
     *
     * @return self
     */
    public static function getInstance(): self
    {
        if (self::$instance === null) {
            self::$instance = new self();
        }

        return self::$instance;
    }

    /**
     * Register a post-processor callback.
     *
     * The callback must accept an ExtractionResult and return an ExtractionResult.
     * It will be called during the extraction pipeline to enrich results.
     *
     * @param string $name Unique name for the post-processor
     * @param callable(ExtractionResult): ExtractionResult $callback Processing callback
     * @return void
     *
     * @throws \Exception If the name is empty, already registered, or callback is invalid
     *
     * @example
     * ```php
     * $registry->register('add_timestamp', function($result) {
     *     $result->metadata->additional['processed_at'] = date('c');
     *     return $result;
     * });
     * ```
     */
    public function register(string $name, callable $callback): void
    {
        kreuzberg_register_post_processor($name, $callback);
    }

    /**
     * Register a post-processor instance implementing PostProcessorInterface.
     *
     * This is a convenience method that wraps a PostProcessorInterface instance
     * in a closure for registration.
     *
     * @param string $name Unique name for the post-processor
     * @param PostProcessorInterface $processor Post-processor instance
     * @return void
     *
     * @throws \Exception If registration fails
     *
     * @example
     * ```php
     * $processor = new WordCountProcessor();
     * $registry->registerInstance('word_count', $processor);
     * ```
     */
    public function registerInstance(string $name, PostProcessorInterface $processor): void
    {
        $this->register($name, [$processor, 'process']);
    }

    /**
     * Unregister a post-processor by name.
     *
     * Removes the post-processor from the registry. After unregistration,
     * it will no longer be called during extraction.
     *
     * @param string $name Name of the post-processor to unregister
     * @return void
     *
     * @throws \Exception If the post-processor is not found
     *
     * @example
     * ```php
     * $registry->unregister('word_count');
     * ```
     */
    public function unregister(string $name): void
    {
        kreuzberg_unregister_post_processor($name);
    }

    /**
     * List all registered post-processor names.
     *
     * Returns an array of all currently registered post-processor names,
     * including both PHP and Rust post-processors.
     *
     * @return array<string> Array of post-processor names
     *
     * @example
     * ```php
     * $processors = $registry->list();
     * foreach ($processors as $name) {
     *     echo "Registered: $name\n";
     * }
     * ```
     */
    public function list(): array
    {
        return kreuzberg_list_post_processors();
    }

    /**
     * Check if a post-processor is registered.
     *
     * @param string $name Post-processor name to check
     * @return bool True if registered, false otherwise
     *
     * @example
     * ```php
     * if ($registry->has('word_count')) {
     *     echo "Word count processor is active\n";
     * }
     * ```
     */
    public function has(string $name): bool
    {
        return in_array($name, $this->list(), true);
    }

    /**
     * Clear all registered post-processors.
     *
     * Removes all post-processors from the registry. This is primarily
     * useful for testing and cleanup.
     *
     * @return void
     *
     * @example
     * ```php
     * // In test cleanup
     * $registry->clear();
     * ```
     */
    public function clear(): void
    {
        kreuzberg_clear_post_processors();
    }

    /**
     * Get the number of registered post-processors.
     *
     * @return int Count of registered post-processors
     *
     * @example
     * ```php
     * $count = $registry->count();
     * echo "Active processors: $count\n";
     * ```
     */
    public function count(): int
    {
        return count($this->list());
    }
}

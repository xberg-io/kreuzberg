<?php

declare(strict_types=1);

namespace Kreuzberg\Plugins;

/**
 * Interface for custom document extractors.
 *
 * Implement this interface to create custom extractors for specific document formats.
 * Extractors are registered by MIME type and are called before built-in extractors.
 *
 * @package Kreuzberg\Plugins
 */
interface ExtractorInterface
{
    /**
     * Extract content from document bytes.
     *
     * This method receives raw document bytes and must return an extraction result
     * containing the extracted text content, metadata, and optional tables.
     *
     * @param string $bytes Raw document bytes
     * @param string $mimeType MIME type of the document
     * @return array{
     *     content: string,
     *     metadata?: array<string, mixed>,
     *     tables?: array<int, array{
     *         cells: array<int, array<int, string>>,
     *         markdown: string,
     *         page_number: int
     *     }>
     * } Extraction result array
     *
     * @throws \RuntimeException If extraction fails
     *
     * @example
     * ```php
     * public function extract(string $bytes, string $mimeType): array
     * {
     *     $content = $this->parseDocument($bytes);
     *
     *     return [
     *         'content' => $content,
     *         'metadata' => [
     *             'extractor' => 'custom',
     *             'version' => '1.0.0',
     *         ],
     *         'tables' => [],
     *     ];
     * }
     * ```
     */
    public function extract(string $bytes, string $mimeType): array;
}

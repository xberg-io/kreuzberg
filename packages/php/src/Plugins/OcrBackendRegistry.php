<?php

declare(strict_types=1);

namespace Kreuzberg\Plugins;

/**
 * Registry for OCR backend plugins.
 *
 * Provides static methods to register, unregister, and list OCR backends.
 * OCR backends are used by the extraction pipeline when processing images.
 *
 * @package Kreuzberg\Plugins
 */
class OcrBackendRegistry
{
    /**
     * Register a custom OCR backend.
     *
     * The backend will be available for use in extraction operations.
     * Backend names must be unique and follow kebab-case convention.
     *
     * The callback must accept two parameters:
     * - string $imageData: Raw image bytes
     * - string $language: Language code (e.g., "eng", "deu")
     *
     * The callback must return an array with at minimum a 'content' key:
     * [
     *     'content' => 'extracted text',
     *     'metadata' => ['key' => 'value'],  // Optional
     *     'tables' => []                      // Optional
     * ]
     *
     * @param string $name Unique backend name (e.g., "my-ocr", "easyocr")
     * @param callable $callback Callback implementing OcrBackendInterface::process()
     *
     * @return void
     *
     * @throws \Exception If registration fails (e.g., name already registered, invalid callback)
     *
     * @example
     * ```php
     * use Kreuzberg\Plugins\OcrBackendRegistry;
     * use Kreuzberg\Plugins\OcrBackendInterface;
     *
     * class MyOcrBackend implements OcrBackendInterface {
     *     public function process(string $imageData, string $language): array {
     *         return [
     *             'content' => 'extracted text',
     *             'metadata' => ['confidence' => 0.95],
     *             'tables' => []
     *         ];
     *     }
     * }
     *
     * $backend = new MyOcrBackend();
     * OcrBackendRegistry::register('my-ocr', [$backend, 'process']);
     * ```
     */
    public static function register(string $name, callable $callback): void
    {
        kreuzberg_register_ocr_backend($name, $callback);
    }

    /**
     * Unregister an OCR backend by name.
     *
     * Removes the backend from the registry and cleans up resources.
     * If the backend is currently in use, the unregistration will succeed
     * but active operations may continue until completion.
     *
     * @param string $name Backend name to unregister
     *
     * @return void
     *
     * @throws \Exception If backend is not found or unregistration fails
     *
     * @example
     * ```php
     * OcrBackendRegistry::unregister('my-ocr');
     * ```
     */
    public static function unregister(string $name): void
    {
        kreuzberg_unregister_ocr_backend($name);
    }

    /**
     * List all registered OCR backend names.
     *
     * Returns names of both Rust-native backends (e.g., tesseract) and
     * PHP custom backends.
     *
     * @return string[] Array of backend names
     *
     * @example
     * ```php
     * $backends = OcrBackendRegistry::list();
     * foreach ($backends as $name) {
     *     echo "Backend: $name\n";
     * }
     * // Output might be:
     * // Backend: tesseract
     * // Backend: my-ocr
     * ```
     */
    public static function list(): array
    {
        return kreuzberg_list_ocr_backends();
    }
}

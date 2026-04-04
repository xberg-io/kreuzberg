<?php

declare(strict_types=1);

namespace Kreuzberg\Plugins;

/**
 * Interface for custom OCR backend implementations.
 *
 * OCR backends process image data and extract text, metadata, and tables.
 * They are called by the extraction pipeline when processing images or
 * when force_ocr is enabled for other document types.
 *
 * @package Kreuzberg\Plugins
 */
interface OcrBackendInterface
{
    /**
     * Process image data and extract text.
     *
     * This method receives raw image bytes and a language code, and must
     * return an array with the extraction results.
     *
     * The returned array must have the following structure:
     * [
     *     'content' => 'extracted text',        // Required: extracted text content
     *     'metadata' => [                       // Optional: metadata about the extraction
     *         'confidence' => 0.95,
     *         'processing_time_ms' => 150,
     *         // ... other metadata fields
     *     ],
     *     'tables' => [                         // Optional: extracted tables
     *         [
     *             'cells' => [                  // Required: 2D array of cell values
     *                 ['Header 1', 'Header 2'],
     *                 ['Cell 1', 'Cell 2']
     *             ],
     *             'markdown' => '| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |',
     *             'page_number' => 1
     *         ]
     *     ]
     * ]
     *
     * @param string $imageData Raw image bytes (PNG, JPEG, etc.)
     * @param string $language Language code (e.g., "eng", "deu", "fra")
     *
     * @return array<string, mixed> Extraction result
     *
     * @throws \Exception If OCR processing fails
     *
     * @example
     * ```php
     * public function process(string $imageData, string $language): array {
     *     // Process image with OCR library
     *     $text = $this->ocrLibrary->recognize($imageData, $language);
     *
     *     return [
     *         'content' => $text,
     *         'metadata' => ['confidence' => 0.95],
     *         'tables' => []
     *     ];
     * }
     * ```
     */
    public function process(string $imageData, string $language): array;
}

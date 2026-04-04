<?php

declare(strict_types=1);

namespace Kreuzberg\Plugins;

use Kreuzberg\Types\ExtractionResult;

/**
 * Interface for post-processor plugins.
 *
 * Post-processors transform or enrich extraction results after the initial
 * extraction is complete. They can:
 * - Clean and normalize text
 * - Add metadata (language, keywords, entities)
 * - Score quality
 * - Apply custom transformations
 *
 * @example Basic post-processor implementation
 * ```php
 * class WordCountProcessor implements PostProcessorInterface
 * {
 *     public function process(ExtractionResult $result): ExtractionResult
 *     {
 *         $wordCount = str_word_count($result->content);
 *
 *         // Clone result and add metadata
 *         $newResult = new ExtractionResult(
 *             content: $result->content,
 *             mimeType: $result->mimeType,
 *             metadata: $result->metadata,
 *             tables: $result->tables,
 *             detectedLanguages: $result->detectedLanguages,
 *             chunks: $result->chunks,
 *             images: $result->images,
 *             pages: $result->pages,
 *         );
 *
 *         // Add word count to metadata
 *         $newResult->metadata->additional['word_count'] = $wordCount;
 *
 *         return $newResult;
 *     }
 * }
 * ```
 */
interface PostProcessorInterface
{
    /**
     * Process and enrich an extraction result.
     *
     * Receives an ExtractionResult and returns a modified ExtractionResult.
     * The processor can:
     * - Modify the content (cleaning, normalization)
     * - Add metadata fields
     * - Extract entities, keywords, etc.
     * - Calculate quality scores
     *
     * @param ExtractionResult $result The extraction result to process
     * @return ExtractionResult The modified extraction result
     *
     * @throws \Exception If processing fails critically
     */
    public function process(ExtractionResult $result): ExtractionResult;
}

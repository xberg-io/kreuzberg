<?php

declare(strict_types=1);

/**
 * Example: Custom Post-Processor Plugin
 *
 * This example demonstrates how to implement and use custom post-processors
 * to transform and enrich extraction results after content extraction.
 *
 * Post-processors receive an ExtractionResult and return a modified ExtractionResult,
 * allowing you to clean content, add metadata, extract entities, and more.
 */

require_once __DIR__ . '/../vendor/autoload.php';

use Kreuzberg\Plugins\PostProcessorInterface;
use Kreuzberg\Plugins\PostProcessorRegistry;
use Kreuzberg\Types\ExtractionResult;
use Kreuzberg\Types\Metadata;

// ============================================================================
// Example 1: Simple Text Normalization Post-Processor
// ============================================================================

/**
 * Post-processor that normalizes extracted text.
 *
 * This demonstrates the minimum required implementation of PostProcessorInterface.
 */
class TextNormalizerProcessor implements PostProcessorInterface
{
    public function process(ExtractionResult $result): ExtractionResult
    {
        // Normalize whitespace: remove extra spaces and trim lines
        $normalized = preg_replace('/\s+/', ' ', $result->content);
        $normalized = trim($normalized);

        return new ExtractionResult(
            content: $normalized,
            mimeType: $result->mimeType,
            metadata: $result->metadata,
            tables: $result->tables,
            detectedLanguages: $result->detectedLanguages,
            chunks: $result->chunks,
            images: $result->images,
            pages: $result->pages,
        );
    }
}

// Register the normalizer
$registry = PostProcessorRegistry::getInstance();
$registry->registerInstance('text_normalizer', new TextNormalizerProcessor());

echo "=== Text Normalization ===\n";
echo "Registered: " . ($registry->has('text_normalizer') ? 'Yes' : 'No') . "\n\n";

// ============================================================================
// Example 2: Closure-based Post-Processor with Word Count
// ============================================================================

// Register post-processor using a closure
$registry->register('word_counter', function (ExtractionResult $result): ExtractionResult {
    $wordCount = str_word_count($result->content);
    $charCount = strlen($result->content);

    // Create new metadata with additional fields
    $newMetadata = new Metadata(
        fileName: $result->metadata->fileName,
        filePath: $result->metadata->filePath,
        fileSize: $result->metadata->fileSize,
        createdAt: $result->metadata->createdAt,
        modifiedAt: $result->metadata->modifiedAt,
        additional: array_merge(
            $result->metadata->additional ?? [],
            [
                'word_count' => $wordCount,
                'char_count' => $charCount,
            ]
        ),
    );

    return new ExtractionResult(
        content: $result->content,
        mimeType: $result->mimeType,
        metadata: $newMetadata,
        tables: $result->tables,
        detectedLanguages: $result->detectedLanguages,
        chunks: $result->chunks,
        images: $result->images,
        pages: $result->pages,
    );
});

echo "=== Word Counter ===\n";
echo "Registered: " . ($registry->has('word_counter') ? 'Yes' : 'No') . "\n\n";

// ============================================================================
// Example 3: Content Quality Scoring Post-Processor
// ============================================================================

/**
 * Post-processor that calculates a content quality score.
 *
 * Evaluates content based on:
 * - Word count (longer content generally higher quality)
 * - Readability (sentence structure)
 * - Metadata presence
 */
class QualityScorerProcessor implements PostProcessorInterface
{
    public function process(ExtractionResult $result): ExtractionResult
    {
        $score = $this->calculateScore($result);

        $newMetadata = new Metadata(
            fileName: $result->metadata->fileName,
            filePath: $result->metadata->filePath,
            fileSize: $result->metadata->fileSize,
            createdAt: $result->metadata->createdAt,
            modifiedAt: $result->metadata->modifiedAt,
            additional: array_merge(
                $result->metadata->additional ?? [],
                [
                    'quality_score' => $score,
                    'quality_level' => $this->getQualityLevel($score),
                ]
            ),
        );

        return new ExtractionResult(
            content: $result->content,
            mimeType: $result->mimeType,
            metadata: $newMetadata,
            tables: $result->tables,
            detectedLanguages: $result->detectedLanguages,
            chunks: $result->chunks,
            images: $result->images,
            pages: $result->pages,
        );
    }

    private function calculateScore(ExtractionResult $result): float
    {
        $score = 0.0;

        // Factor 1: Content length (0-30 points)
        $wordCount = str_word_count($result->content);
        $lengthScore = min(30, ($wordCount / 100) * 30);
        $score += $lengthScore;

        // Factor 2: Metadata presence (0-20 points)
        $metadataScore = 0;
        if (!empty($result->metadata->fileName)) {
            $metadataScore += 10;
        }
        if (!empty($result->metadata->createdAt)) {
            $metadataScore += 10;
        }
        $score += $metadataScore;

        // Factor 3: Table presence (0-20 points)
        if (!empty($result->tables)) {
            $score += min(20, count($result->tables) * 5);
        }

        // Factor 4: Detected languages (0-15 points)
        if (!empty($result->detectedLanguages)) {
            $score += 15;
        }

        // Factor 5: Images presence (0-15 points)
        if (!empty($result->images)) {
            $score += min(15, count($result->images) * 3);
        }

        return min(100.0, $score);
    }

    private function getQualityLevel(float $score): string
    {
        if ($score >= 80) {
            return 'excellent';
        } elseif ($score >= 60) {
            return 'good';
        } elseif ($score >= 40) {
            return 'fair';
        } else {
            return 'poor';
        }
    }
}

$registry->registerInstance('quality_scorer', new QualityScorerProcessor());

echo "=== Quality Scorer ===\n";
echo "Registered: " . ($registry->has('quality_scorer') ? 'Yes' : 'No') . "\n\n";

// ============================================================================
// Example 4: Content Filtering Post-Processor
// ============================================================================

/**
 * Post-processor that filters out unwanted patterns from content.
 *
 * Removes or cleans:
 * - URLs and email addresses
 * - HTML-like tags
 * - Special characters
 */
class ContentFilterProcessor implements PostProcessorInterface
{
    public function process(ExtractionResult $result): ExtractionResult
    {
        $filtered = $result->content;

        // Remove URLs
        $filtered = preg_replace(
            '/(https?:\/\/[^\s]+|www\.[^\s]+)/',
            '',
            $filtered
        );

        // Remove email addresses
        $filtered = preg_replace(
            '/([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})/',
            '',
            $filtered
        );

        // Remove HTML-like tags
        $filtered = preg_replace('/<[^>]+>/', '', $filtered);

        // Clean up multiple spaces
        $filtered = preg_replace('/\s+/', ' ', $filtered);
        $filtered = trim($filtered);

        return new ExtractionResult(
            content: $filtered,
            mimeType: $result->mimeType,
            metadata: $result->metadata,
            tables: $result->tables,
            detectedLanguages: $result->detectedLanguages,
            chunks: $result->chunks,
            images: $result->images,
            pages: $result->pages,
        );
    }
}

$registry->registerInstance('content_filter', new ContentFilterProcessor());

echo "=== Content Filter ===\n";
echo "Registered: " . ($registry->has('content_filter') ? 'Yes' : 'No') . "\n\n";

// ============================================================================
// Example 5: Listing and Managing Registered Post-Processors
// ============================================================================

echo "=== Registered Post-Processors ===\n";
$processors = $registry->list();
foreach ($processors as $name) {
    echo "- {$name}\n";
}
echo "Total: " . count($processors) . "\n\n";

// ============================================================================
// Example 6: Chaining Multiple Post-Processors
// ============================================================================

echo "=== Post-Processor Chaining ===\n";
echo "Post-processors are applied in registration order.\n";
echo "For example:\n";
echo "1. content_filter - Removes URLs, emails, tags\n";
echo "2. text_normalizer - Normalizes whitespace\n";
echo "3. word_counter - Counts words and characters\n";
echo "4. quality_scorer - Calculates quality score\n\n";

// ============================================================================
// Example 7: Unregistering Post-Processors
// ============================================================================

echo "=== Unregistering Post-Processors ===\n";
$registry->unregister('word_counter');
echo "Unregistered 'word_counter'\n";
echo "Remaining: " . $registry->count() . "\n\n";

// Re-register for completeness
$registry->register('word_counter', function (ExtractionResult $result): ExtractionResult {
    $wordCount = str_word_count($result->content);
    $charCount = strlen($result->content);

    $newMetadata = new Metadata(
        fileName: $result->metadata->fileName,
        filePath: $result->metadata->filePath,
        fileSize: $result->metadata->fileSize,
        createdAt: $result->metadata->createdAt,
        modifiedAt: $result->metadata->modifiedAt,
        additional: array_merge(
            $result->metadata->additional ?? [],
            [
                'word_count' => $wordCount,
                'char_count' => $charCount,
            ]
        ),
    );

    return new ExtractionResult(
        content: $result->content,
        mimeType: $result->mimeType,
        metadata: $newMetadata,
        tables: $result->tables,
        detectedLanguages: $result->detectedLanguages,
        chunks: $result->chunks,
        images: $result->images,
        pages: $result->pages,
    );
});

// ============================================================================
// Cleanup
// ============================================================================

echo "=== Cleanup ===\n";
// You can clear all post-processors at once
// $registry->clear();
// echo "Cleared all post-processors\n";

// Or check specific ones
if ($registry->has('text_normalizer')) {
    echo "text_normalizer is still registered\n";
}

echo "\nExample completed successfully!\n";

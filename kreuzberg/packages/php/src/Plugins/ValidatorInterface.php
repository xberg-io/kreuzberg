<?php

declare(strict_types=1);

namespace Kreuzberg\Plugins;

/**
 * Interface for validator plugins.
 *
 * Validators check extraction results after extraction but before returning to the user.
 * They can enforce quality standards, content requirements, or business rules.
 *
 * All registered validators must pass for extraction to succeed. If any validator
 * fails (returns false or throws ValidationError), the extraction fails.
 *
 * @example
 * ```php
 * use Kreuzberg\Plugins\ValidatorInterface;
 * use Kreuzberg\Plugins\ValidationError;
 *
 * class MinLengthValidator implements ValidatorInterface
 * {
 *     public function __construct(private int $minLength = 100) {}
 *
 *     public function validate(array $result): bool
 *     {
 *         $contentLength = strlen($result['content']);
 *
 *         if ($contentLength < $this->minLength) {
 *             throw new ValidationError(
 *                 sprintf(
 *                     'Content too short: %d < %d characters',
 *                     $contentLength,
 *                     $this->minLength
 *                 )
 *             );
 *         }
 *
 *         return true;
 *     }
 * }
 *
 * // Register validator
 * $validator = new MinLengthValidator(minLength: 200);
 * kreuzberg_register_validator('min_length', [$validator, 'validate']);
 * ```
 */
interface ValidatorInterface
{
    /**
     * Validate an extraction result.
     *
     * This method receives the extraction result as an array and should:
     * - Return `true` if validation passes
     * - Return `false` if validation fails (generic failure)
     * - Throw `ValidationError` with details if validation fails (preferred)
     *
     * The result array has the following structure:
     * ```php
     * [
     *     'content' => string,           // Extracted text content
     *     'mime_type' => string,         // MIME type of the document
     *     'metadata' => [                // Document metadata
     *         'title' => ?string,
     *         'author' => ?string,
     *         'subject' => ?string,
     *         // ... additional metadata fields
     *     ],
     *     'tables' => array,             // Extracted tables
     *     'detected_languages' => ?array, // Detected language codes
     *     'chunks' => ?array,            // Text chunks with embeddings
     *     'images' => ?array,            // Extracted images
     *     'pages' => ?array,             // Per-page content
     * ]
     * ```
     *
     * @param array<string, mixed> $result The extraction result to validate
     * @return bool True if validation passes, false if it fails
     * @throws ValidationError If validation fails with specific error details
     *
     * @example
     * ```php
     * public function validate(array $result): bool
     * {
     *     // Check content length
     *     if (strlen($result['content']) < 100) {
     *         throw new ValidationError('Content too short');
     *     }
     *
     *     // Check for required metadata
     *     if (empty($result['metadata']['title'])) {
     *         throw new ValidationError('Missing required title metadata');
     *     }
     *
     *     // Check detected languages
     *     if (isset($result['detected_languages'])) {
     *         $allowedLanguages = ['en', 'de', 'fr'];
     *         $detected = $result['detected_languages'];
     *
     *         if (!array_intersect($detected, $allowedLanguages)) {
     *             throw new ValidationError(
     *                 'Detected languages not supported: ' . implode(', ', $detected)
     *             );
     *         }
     *     }
     *
     *     return true;
     * }
     * ```
     */
    public function validate(array $result): bool;
}

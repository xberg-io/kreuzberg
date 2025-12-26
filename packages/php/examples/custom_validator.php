<?php

declare(strict_types=1);

/**
 * Example: Custom Validator Plugin
 *
 * This example demonstrates how to implement and use custom validators
 * to enforce quality standards and validation rules on extraction results.
 */

require_once __DIR__ . '/../vendor/autoload.php';

use Kreuzberg\Plugins\ValidatorInterface;
use Kreuzberg\Plugins\ValidatorRegistry;
use Kreuzberg\Plugins\ValidationError;

// ============================================================================
// Example 1: Minimum Content Length Validator
// ============================================================================

/**
 * Validator that ensures extracted content meets a minimum length requirement.
 *
 * This is a simple validator that checks if the content has at least
 * a specified number of characters.
 */
class MinLengthValidator implements ValidatorInterface
{
    public function __construct(private int $minLength = 100)
    {
    }

    public function validate(array $result): bool
    {
        $contentLength = strlen($result['content']);

        if ($contentLength < $this->minLength) {
            throw ValidationError::contentTooShort(
                actual: $contentLength,
                minimum: $this->minLength,
                validator: 'min_length'
            );
        }

        return true;
    }
}

// Register the minimum length validator
$registry = ValidatorRegistry::getInstance();
$registry->register('min_length', new MinLengthValidator(minLength: 100));

// Test the validator registration
echo "=== Registered Validators ===\n";
$validators = $registry->list();
foreach ($validators as $name) {
    echo "- {$name}\n";
}
echo "\n";

// ============================================================================
// Example 2: Required Metadata Validator
// ============================================================================

/**
 * Validator that ensures required metadata fields are present.
 */
class RequiredMetadataValidator implements ValidatorInterface
{
    public function __construct(private array $requiredFields = ['title'])
    {
    }

    public function validate(array $result): bool
    {
        $metadata = $result['metadata'] ?? [];

        foreach ($this->requiredFields as $field) {
            if (empty($metadata[$field])) {
                throw ValidationError::missingField(
                    field: $field,
                    validator: 'required_metadata'
                );
            }
        }

        return true;
    }
}

$registry->register('required_metadata', new RequiredMetadataValidator(
    requiredFields: ['title']
));

// ============================================================================
// Example 3: Language Detection Validator
// ============================================================================

/**
 * Validator that checks if detected languages are in an allowed list.
 */
class LanguageValidator implements ValidatorInterface
{
    public function __construct(private array $allowedLanguages = ['en', 'de', 'fr'])
    {
    }

    public function validate(array $result): bool
    {
        $detectedLanguages = $result['detected_languages'] ?? [];

        if (empty($detectedLanguages)) {
            // If no languages detected, skip validation
            return true;
        }

        // Check if any detected language is in the allowed list
        $hasAllowedLanguage = !empty(array_intersect($detectedLanguages, $this->allowedLanguages));

        if (!$hasAllowedLanguage) {
            throw new ValidationError(
                message: 'Detected languages not in allowed list',
                details: [
                    'field' => 'detected_languages',
                    'detected' => $detectedLanguages,
                    'allowed' => $this->allowedLanguages,
                    'error' => 'unsupported_language',
                    'validator' => 'language_validator',
                ]
            );
        }

        return true;
    }
}

$registry->register('language_validator', new LanguageValidator(
    allowedLanguages: ['en', 'de', 'fr']
));

// ============================================================================
// Example 4: Closure-based Validator
// ============================================================================

// Register a simple closure validator
$registry->register('has_content', function (array $result): bool {
    if (empty($result['content'])) {
        throw new ValidationError(
            message: 'Extraction resulted in empty content',
            details: [
                'field' => 'content',
                'error' => 'empty_content',
                'validator' => 'has_content',
            ]
        );
    }
    return true;
});

// ============================================================================
// Example 5: Complex Validator with Multiple Checks
// ============================================================================

/**
 * Comprehensive validator that checks multiple aspects of the extraction result.
 */
class ComprehensiveValidator implements ValidatorInterface
{
    public function __construct(
        private int $minContentLength = 50,
        private int $maxContentLength = 100000,
        private int $minTables = 0,
    ) {
    }

    public function validate(array $result): bool
    {
        $contentLength = strlen($result['content']);
        $tableCount = count($result['tables'] ?? []);

        // Check content length
        if ($contentLength < $this->minContentLength) {
            throw ValidationError::contentTooShort(
                actual: $contentLength,
                minimum: $this->minContentLength,
                validator: 'comprehensive'
            );
        }

        if ($contentLength > $this->maxContentLength) {
            throw ValidationError::contentTooLong(
                actual: $contentLength,
                maximum: $this->maxContentLength,
                validator: 'comprehensive'
            );
        }

        // Check table count
        if ($tableCount < $this->minTables) {
            throw ValidationError::invalidValue(
                field: 'tables.count',
                actual: $tableCount,
                expected: '>= ' . $this->minTables,
                validator: 'comprehensive'
            );
        }

        return true;
    }
}

$registry->register('comprehensive', new ComprehensiveValidator(
    minContentLength: 50,
    maxContentLength: 100000,
    minTables: 0
));

// ============================================================================
// Example 6: Testing Validators with Mock Data
// ============================================================================

echo "=== Testing Validators ===\n";

// Create mock extraction results
$validResult = [
    'content' => str_repeat('Sample content ', 10), // ~150 chars
    'mime_type' => 'application/pdf',
    'metadata' => [
        'title' => 'Sample Document',
        'author' => 'John Doe',
    ],
    'tables' => [],
    'detected_languages' => ['en'],
];

$invalidResultShortContent = [
    'content' => 'Too short',
    'mime_type' => 'application/pdf',
    'metadata' => [
        'title' => 'Sample',
    ],
    'tables' => [],
    'detected_languages' => ['en'],
];

$invalidResultMissingMetadata = [
    'content' => str_repeat('Content ', 20),
    'mime_type' => 'application/pdf',
    'metadata' => [], // Missing title
    'tables' => [],
    'detected_languages' => ['en'],
];

// Test with valid result
echo "\n--- Testing Valid Result ---\n";
try {
    // Get the min_length validator and test it
    $validator = new MinLengthValidator(100);
    $result = $validator->validate($validResult);
    echo "Min length validation: PASSED\n";
} catch (ValidationError $e) {
    echo "Min length validation: FAILED\n";
    echo "Error: " . $e->getMessage() . "\n";
}

try {
    $validator = new RequiredMetadataValidator(['title']);
    $result = $validator->validate($validResult);
    echo "Required metadata validation: PASSED\n";
} catch (ValidationError $e) {
    echo "Required metadata validation: FAILED\n";
    echo "Error: " . $e->getMessage() . "\n";
}

// Test with invalid result (short content)
echo "\n--- Testing Invalid Result (Short Content) ---\n";
try {
    $validator = new MinLengthValidator(100);
    $result = $validator->validate($invalidResultShortContent);
    echo "Min length validation: PASSED\n";
} catch (ValidationError $e) {
    echo "Min length validation: FAILED\n";
    echo "Error: " . $e->getMessage() . "\n";
    echo "Details: " . json_encode($e->getDetails(), JSON_PRETTY_PRINT) . "\n";
}

// Test with invalid result (missing metadata)
echo "\n--- Testing Invalid Result (Missing Metadata) ---\n";
try {
    $validator = new RequiredMetadataValidator(['title']);
    $result = $validator->validate($invalidResultMissingMetadata);
    echo "Required metadata validation: PASSED\n";
} catch (ValidationError $e) {
    echo "Required metadata validation: FAILED\n";
    echo "Error: " . $e->getMessage() . "\n";
    echo "Details: " . json_encode($e->getDetails(), JSON_PRETTY_PRINT) . "\n";
}

// ============================================================================
// Example 7: Scoped Validator Registration (Temporary Validators)
// ============================================================================

echo "\n=== Scoped Validator Registration ===\n";

$beforeCount = $registry->count();
echo "Validators before withValidators: {$beforeCount}\n";

try {
    $result = $registry->withValidators(
        validators: [
            'temp_validator' => function (array $result): bool {
                return strlen($result['content']) > 0;
            },
        ],
        callback: function () use ($registry) {
            $countDuring = $registry->count();
            echo "Validators during withValidators: {$countDuring}\n";
            return true;
        }
    );
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}

$afterCount = $registry->count();
echo "Validators after withValidators: {$afterCount}\n";
echo "Temporary validator was cleaned up: " . ($afterCount === $beforeCount ? "YES" : "NO") . "\n";

// ============================================================================
// Example 8: Method Chaining for Multiple Registrations
// ============================================================================

echo "\n=== Validator Registry Summary ===\n";

// List all currently registered validators
$allValidators = $registry->list();
echo "Total validators registered: " . count($allValidators) . "\n";
echo "Registered validator names:\n";
foreach ($allValidators as $name) {
    echo "  - {$name}\n";
}

// ============================================================================
// Cleanup
// ============================================================================

echo "\n=== Cleanup ===\n";

// Unregister specific validators
$registry->unregister('has_content');
echo "Unregistered 'has_content' validator\n";

// Or clear all validators
// $registry->clear();
// echo "Cleared all validators\n";

echo "Remaining validators: " . count($registry->list()) . "\n";

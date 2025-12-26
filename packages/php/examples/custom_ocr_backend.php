<?php

/**
 * Example: Custom OCR Backend Implementation
 *
 * This example demonstrates how to create and register a custom OCR backend
 * for use with Kreuzberg's extraction pipeline.
 */

require_once __DIR__ . '/../vendor/autoload.php';

use Kreuzberg\Plugins\OcrBackendInterface;
use Kreuzberg\Plugins\OcrBackendRegistry;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\OcrConfig;

/**
 * Simple OCR backend that returns mock data.
 *
 * In a real implementation, you would integrate with an OCR library
 * such as Tesseract, EasyOCR, PaddleOCR, or a cloud service.
 */
class SimpleOcrBackend implements OcrBackendInterface
{
    public function process(string $imageData, string $language): array
    {
        // In a real implementation:
        // 1. Save $imageData to a temporary file or process in memory
        // 2. Call your OCR library with the image and language
        // 3. Extract text, confidence scores, and optionally tables
        // 4. Return formatted results

        $text = $this->performOcr($imageData, $language);

        return [
            'content' => $text,
            'metadata' => [
                'confidence' => 0.95,
                'processing_time_ms' => 120,
                'engine' => 'simple-ocr-v1',
                'language' => $language,
            ],
            'tables' => []  // No table detection in this simple example
        ];
    }

    private function performOcr(string $imageData, string $language): string
    {
        // Mock OCR processing
        // Replace this with actual OCR library calls
        return "This is mock OCR output for a {$language} image.";
    }
}

/**
 * Advanced OCR backend with table detection.
 *
 * This example shows how to return table data in the correct format.
 */
class AdvancedOcrBackend implements OcrBackendInterface
{
    public function process(string $imageData, string $language): array
    {
        $text = $this->performOcr($imageData, $language);
        $tables = $this->detectTables($imageData);

        return [
            'content' => $text,
            'metadata' => [
                'confidence' => 0.92,
                'has_tables' => count($tables) > 0,
                'table_count' => count($tables),
            ],
            'tables' => $tables
        ];
    }

    private function performOcr(string $imageData, string $language): string
    {
        return "Invoice\nDate: 2024-01-15\nAmount: $150.00";
    }

    private function detectTables(string $imageData): array
    {
        // Mock table detection
        // In production, use a table detection library or model
        return [
            [
                'cells' => [
                    ['Item', 'Quantity', 'Price'],
                    ['Widget A', '2', '$50.00'],
                    ['Widget B', '1', '$50.00'],
                ],
                'markdown' => "| Item     | Quantity | Price  |\n|----------|----------|--------|\n| Widget A | 2        | $50.00 |\n| Widget B | 1        | $50.00 |",
                'page_number' => 1
            ]
        ];
    }
}

// Example 1: Register and use simple OCR backend
echo "=== Example 1: Simple OCR Backend ===\n";

$simpleBackend = new SimpleOcrBackend();
OcrBackendRegistry::register(
    'simple-ocr',
    [$simpleBackend, 'process'],
    ['eng', 'deu', 'fra']
);

// List all backends
$backends = OcrBackendRegistry::list();
echo "Registered backends: " . implode(', ', $backends) . "\n";

// Use the backend in extraction
$config = new ExtractionConfig();
$config->ocr = new OcrConfig();
$config->ocr->backend = 'simple-ocr';
$config->ocr->language = 'eng';

// Extract from an image (replace with actual image path)
// $result = kreuzberg_extract_file('invoice.png', null, $config);
// echo "Extracted content: " . $result->content . "\n";

// Unregister when done
OcrBackendRegistry::unregister('simple-ocr');

echo "\n";

// Example 2: Register and use advanced OCR backend with tables
echo "=== Example 2: Advanced OCR Backend with Tables ===\n";

$advancedBackend = new AdvancedOcrBackend();
OcrBackendRegistry::register(
    'advanced-ocr',
    [$advancedBackend, 'process'],
    ['eng', 'deu', 'fra', 'spa', 'ita']
);

$config->ocr->backend = 'advanced-ocr';

// Extract from an image
// $result = kreuzberg_extract_file('document.png', null, $config);
// echo "Extracted content: " . $result->content . "\n";
// echo "Tables found: " . count($result->tables) . "\n";
// foreach ($result->tables as $table) {
//     echo "Table on page {$table->page_number}:\n";
//     echo $table->markdown . "\n";
// }

OcrBackendRegistry::unregister('advanced-ocr');

echo "\n";

// Example 3: Using a closure as callback
echo "=== Example 3: Closure-based OCR Backend ===\n";

$ocrCallback = function(string $imageData, string $language): array {
    return [
        'content' => "Simple closure-based OCR result",
        'metadata' => ['type' => 'closure'],
        'tables' => []
    ];
};

OcrBackendRegistry::register('closure-ocr', $ocrCallback, ['eng']);

echo "Registered closure-based backend\n";

OcrBackendRegistry::unregister('closure-ocr');

echo "\n";

// Example 4: Error handling
echo "=== Example 4: Error Handling ===\n";

class ErrorHandlingBackend implements OcrBackendInterface
{
    public function process(string $imageData, string $language): array
    {
        try {
            if (strlen($imageData) < 100) {
                throw new \RuntimeException('Image data too small');
            }

            // Process image...
            $text = "Processed successfully";

            return [
                'content' => $text,
                'metadata' => [],
                'tables' => []
            ];
        } catch (\Exception $e) {
            // Log the error
            error_log("OCR error: " . $e->getMessage());

            // Re-throw to let Kreuzberg handle it
            throw new \Exception("OCR processing failed: " . $e->getMessage());
        }
    }
}

$errorBackend = new ErrorHandlingBackend();
OcrBackendRegistry::register(
    'error-backend',
    [$errorBackend, 'process'],
    ['eng']
);

echo "Registered error-handling backend\n";

// This would throw an exception due to small image data:
// try {
//     $result = kreuzberg_extract_bytes('tiny', 'image/png', $config);
// } catch (\Exception $e) {
//     echo "Caught exception: " . $e->getMessage() . "\n";
// }

OcrBackendRegistry::unregister('error-backend');

echo "\n";
echo "All examples completed successfully!\n";

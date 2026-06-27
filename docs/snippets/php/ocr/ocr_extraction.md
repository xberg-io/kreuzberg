```php title="PHP"
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\OcrConfig;

// Basic OCR extraction with Tesseract
$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng'
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('scanned_document.pdf'), $config);
$result = $resultOutput->results[0];

echo "Extracted Text:\n";
echo str_repeat('=', 60) . "\n";
echo $result->content . "\n\n";

echo "Extraction Metadata:\n";
echo "Page count: " . ($result->metadata?->pdf?->page_count ?? 'unknown') . "\n";
echo "Characters: " . strlen($result->content) . "\n";
echo "Tables found: " . count($result->tables) . "\n";

// Extract from image
if (file_exists('scanned_image.png')) {
    $imageResultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('scanned_image.png'), $config);
    $imageResult = $imageResultOutput->results[0];
    echo "\nImage OCR Results:\n";
    echo $imageResult->content . "\n";
}
?>
```

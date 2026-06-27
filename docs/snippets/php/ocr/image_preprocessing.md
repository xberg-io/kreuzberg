```php title="PHP"
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\OcrConfig;
use Xberg\ImagePreprocessingConfig;

// Enhance OCR accuracy with image preprocessing
$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng',
        imagePreprocessing: new ImagePreprocessingConfig(
            targetDpi: 300,
            autoRotate: true,
            deskew: true,
            denoise: true,
            contrastEnhance: true,
            binarizationMethod: 'otsu',
            invertColors: false
        )
    )
);

$output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri('scanned_document.pdf'), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

echo "Preprocessed OCR Results:\n";
echo "Characters extracted: " . strlen($result->content) . "\n";
echo "Preview: " . substr($result->content, 0, 300) . "...\n";
?>
```

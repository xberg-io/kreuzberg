```php title="PHP"
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\OcrConfig;

// Extract text from multilingual documents
// Specify multiple language codes separated by plus (+)
$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng+fra+deu'  // English, French, German
    )
);

$output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri('multilingual_document.pdf'), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

echo "Multilingual OCR Results:\n";
echo "Supported languages: English, French, German\n";
echo "Extracted content:\n";
echo $result->content . "\n\n";

// Language detection with multi-language support
$autoDetectConfig = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng+spa+fra+deu+ita+por'  // Multiple European languages
    )
);

$output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri('european_document.pdf'), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

echo "European Language Document:\n";
echo "Extracted " . strlen($result->content) . " characters\n";
echo "Preview: " . substr($result->content, 0, 300) . "...\n\n";

// Mixed language with language detection
$mixedConfig = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng+jpn+chi_sim'  // English, Japanese, Chinese Simplified
    )
);

$output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri('asian_document.pdf'), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

echo "Multi-script Document:\n";
echo "Characters extracted: " . mb_strlen($result->content) . "\n";
?>
```

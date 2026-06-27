```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\OcrConfig;

$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'tesseract',
        language: 'eng'
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('scanned.pdf'), $config);

$result = $resultOutput->results[0];

echo "Content length: " . strlen($result->content) . " characters\n";
echo "Tables detected: " . count($result->tables) . "\n";
?>
```

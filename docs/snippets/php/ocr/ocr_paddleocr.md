```php title="PHP"
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\OcrConfig;

$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'paddle-ocr',
        language: 'en',
        // paddleOcrConfig: new PaddleOcrConfig(modelTier: 'server') // for max accuracy
    )
);

$output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri('scanned_document.pdf'), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

echo $result->content . "\n";
```

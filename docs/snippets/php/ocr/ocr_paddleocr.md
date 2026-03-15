```php title="PHP"
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Kreuzberg\Kreuzberg;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\OcrConfig;

$config = new ExtractionConfig(
    ocr: new OcrConfig(
        backend: 'paddle-ocr',
        language: 'en',
        paddleOcrConfig: new PaddleOcrConfig(
            modelTier: 'mobile'
        )
    )
);

$kreuzberg = new Kreuzberg($config);
$result = $kreuzberg->extractFile('scanned_document.pdf');

echo $result->content . "\n";
```

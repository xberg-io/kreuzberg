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
        language: 'en'
    )
);

$kreuzberg = new Kreuzberg($config);
$result = $kreuzberg->extractFile('scanned.pdf');

if ($result->ocrElements !== null) {
    foreach ($result->ocrElements as $element) {
        echo "Text: {$element->text}\n";
        echo "Confidence: " . number_format($element->confidence->recognition, 2) . "\n";
        echo "Geometry: " . json_encode($element->geometry) . "\n";
        if ($element->rotation !== null) {
            echo "Rotation: {$element->rotation->angle}°\n";
        }
        echo "\n";
    }
}
```

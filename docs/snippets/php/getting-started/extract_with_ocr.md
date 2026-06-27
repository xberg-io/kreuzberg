```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\OcrConfig;

$ocrConfig = new OcrConfig();
$ocrConfig->setBackend('tesseract');
$ocrConfig->setLanguage('eng');

$config = ExtractionConfig::default();
$config->setForceOcr(true);
$config->setOcr($ocrConfig);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('scanned.pdf'), $config);

$result = $resultOutput->results[0];

echo "Content:\n";
echo $result->content;

if ($result->getDetectedLanguages() !== null) {
    echo "Detected Languages: " . implode(', ', $result->getDetectedLanguages()) . "\n";
}
```

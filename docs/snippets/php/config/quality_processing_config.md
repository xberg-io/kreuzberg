```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;

$config = new ExtractionConfig(
    enableQualityProcessing: true,
    useCache: true
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);

$result = $resultOutput->results[0];

echo "Quality score: " . $result->getQualityScore() . "\n";
if ($result->getProcessingTime()) {
    echo "Processing time: " . $result->getProcessingTime() . "ms\n";
}
?>
```

```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;

$config = new ExtractionConfig(
    useCache: true,
    enableQualityProcessing: true
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);

$result = $resultOutput->results[0];

echo $result->content;
?>
```

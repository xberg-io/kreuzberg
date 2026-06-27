```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;

$config = ExtractionConfig::default();
$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);
$result = $resultOutput->results[0];

echo "Content:\n";
echo $result->content;
```

```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), \Xberg\ExtractionConfig::default());

$result = $resultOutput->results[0];
echo "Hello, " . substr($result->content, 0, 50) . "\n";
```

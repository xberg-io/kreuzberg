```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;

// Discover configuration from file system
$config = ExtractionConfig::discover() ?? ExtractionConfig::default();
$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);
$result = $resultOutput->results[0];

echo $result->content;
?>
```

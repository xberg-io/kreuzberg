```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\Xberg;

// Discover configuration from file system
$config = ExtractionConfig::discover() ?? new ExtractionConfig();
$result = Xberg::extractFileSync('document.pdf', null, $config);

echo $result->getContent();
?>
```

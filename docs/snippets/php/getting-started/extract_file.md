```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\Xberg;
use Xberg\ExtractionConfig;

$config = new ExtractionConfig();
$result = Xberg::extractFileSync('document.pdf', null, $config);

echo "Content: " . $result->getContent() . "\n";
echo "MIME Type: " . $result->getMimeType() . "\n";
echo "Tables: " . count($result->getTables()) . "\n";
```

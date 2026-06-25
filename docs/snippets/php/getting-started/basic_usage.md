```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\Xberg;
use Xberg\ExtractionConfig;

$config = new ExtractionConfig();
$result = Xberg::extractFileSync('document.pdf', null, $config);

echo "Content:\n";
echo $result->getContent();
```

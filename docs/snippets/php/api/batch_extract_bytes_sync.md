```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\Xberg;
use Xberg\ExtractionConfig;
use Xberg\BatchBytesItem;

$config = new ExtractionConfig();
$items = [
    new BatchBytesItem('Hello, world!', 'text/plain'),
    new BatchBytesItem("# Heading\n\nParagraph text.", 'text/markdown'),
];
$results = Xberg::batchExtractBytesSync($items, $config);

foreach ($results as $i => $result) {
    echo "Item $i: " . strlen($result->getContent()) . " chars\n";
}
```

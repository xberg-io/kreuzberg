```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\Xberg;
use Xberg\ExtractionConfig;
use Xberg\HtmlOutputConfig;

$config = new ExtractionConfig(
    resultFormat: 'html',
    htmlOutput: new HtmlOutputConfig(
        theme: 'github'
    )
);

$result = Xberg::extractFileSync('document.pdf', null, $config);

// Output HTML with kb-* CSS classes
echo $result->getContent();
?>
```

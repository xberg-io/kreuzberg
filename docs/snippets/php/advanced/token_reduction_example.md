```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\TokenReductionOptions;

$config = new ExtractionConfig(
    tokenReduction: new TokenReductionOptions(
        mode: 'moderate',
        preserveImportantWords: true
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('verbose_document.pdf'), $config);

$result = $resultOutput->results[0];

if ($result->getTokenCount() !== null) {
    echo "Original token count: " . $result->getTokenCount() . "\n";
}

// Access the reduced content
echo "Reduced content length: " . strlen($result->content) . " characters\n";
echo "Content preview: " . substr($result->content, 0, 100) . "...\n";
?>
```

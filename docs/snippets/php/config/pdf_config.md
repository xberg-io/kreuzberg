```php title="PHP"
<?php

declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\PdfConfig;

/**
 * PDF configuration with hierarchy detection
 */
$config = new ExtractionConfig(
    pdf: new PdfConfig(
        extractImages: true,
        extractMetadata: true,
        passwords: ['password1', 'password2'],
        hierarchy: [
            'enabled' => true,
            'k_clusters' => 6,
            'include_bbox' => true,
            'ocr_coverage_threshold' => 0.5
        ]
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);
$result = $resultOutput->results[0];

echo "Content length: " . strlen($result->content) . " characters\n";
echo "Metadata: " . implode(', ', array_keys((array) ($result->metadata?->pdf ?? []))) . "\n";
```

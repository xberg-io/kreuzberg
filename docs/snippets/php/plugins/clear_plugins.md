```php title="PHP"
<?php declare(strict_types=1);

use Xberg\XbergApi;

// Clear all registered OCR backends
Xberg::clearOcrBackends();

// Clear all registered post-processors
Xberg::clearPostProcessors();

// Clear all registered validators
Xberg::clearValidators();

echo "All plugins cleared\n";
```

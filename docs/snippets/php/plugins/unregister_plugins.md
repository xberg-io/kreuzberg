```php title="PHP"
<?php declare(strict_types=1);

use Xberg\XbergApi;

// Unregister all OCR backends by clearing the registry
Xberg::clearOcrBackends();

// Unregister all post-processors by clearing the registry
Xberg::clearPostProcessors();

// Unregister all validators by clearing the registry
Xberg::clearValidators();

echo "All plugins unregistered\n";
```

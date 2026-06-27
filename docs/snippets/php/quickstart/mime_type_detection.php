```php title="mime_type_detection.php"
<?php

declare(strict_types=1);

/**
 * MIME Type Detection
 *
 * Xberg can automatically detect MIME types from file content or paths.
 * This is useful when the file extension is missing or unreliable.
 */

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\ExtractionConfig;

$path = 'document.pdf';
echo "Processing file: $path\n";
echo "MIME type is automatically detected from extension and content.\n\n";

$unknownFile = 'file_without_extension';
if (file_exists($unknownFile)) {
    echo "Processing unknown file: $unknownFile\n";
    echo "ExtractInput will auto-detect the format...\n";

    $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($unknownFile), new ExtractionConfig());
    $result = $output->results[0];
    echo "Successfully extracted " . strlen($result->content) . " characters\n";
}

$allowedExtensions = ['pdf', 'docx', 'txt'];

$fileToCheck = 'user_upload.dat';
if (file_exists($fileToCheck)) {
    $extension = strtolower(pathinfo($fileToCheck, PATHINFO_EXTENSION));

    if (in_array($extension, $allowedExtensions, true)) {
        echo "File extension .$extension is allowed, processing...\n";
        $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($fileToCheck), new ExtractionConfig());
        $result = $output->results[0];
        echo "Extraction successful: " . strlen($result->content) . " characters extracted\n";
    } else {
        echo "File extension .$extension is not allowed\n";
    }
}
```

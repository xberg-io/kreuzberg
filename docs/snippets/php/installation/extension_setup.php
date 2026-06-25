```php title="extension_setup.php"
<?php

declare(strict_types=1);

/**
 * Setting up the Xberg PHP Extension
 *
 * The Xberg native extension must be installed and loaded before using the library.
 * This snippet shows how to check for the extension and provides guidance for installation.
 */

if (!extension_loaded('xberg')) {
    echo "Xberg extension not found!\n\n";
    echo "Installation steps:\n";
    echo "1. Download the extension for your platform from:\n";
    echo "   https://github.com/xberg-io/xberg/releases\n\n";
    echo "2. Copy the extension to your PHP extensions directory:\n";
    echo "   - Linux/macOS: xberg.so\n";
    echo "   - Windows: xberg.dll\n\n";
    echo "3. Add to your php.ini:\n";
    echo "   extension=xberg.so  ; Linux/macOS\n";
    echo "   extension=xberg.dll ; Windows\n\n";
    echo "4. Restart PHP/PHP-FPM/Apache\n\n";
    echo "5. Verify with: php -m | grep xberg\n";
    exit(1);
}

echo "Xberg Extension Information:\n";
echo "================================\n";
echo "Status: Loaded\n";

$tesseract_available = function_exists('xberg_has_tesseract') ? xberg_has_tesseract() : false;
$onnx_available = function_exists('xberg_has_onnx') ? xberg_has_onnx() : false;

echo "Tesseract OCR: " . ($tesseract_available ? "Available" : "Not available") . "\n";
echo "ONNX Runtime: " . ($onnx_available ? "Available" : "Not available") . "\n";

if (!$tesseract_available) {
    echo "\nTo enable OCR functionality, install Tesseract:\n";
    echo "  macOS: brew install tesseract\n";
    echo "  Ubuntu/Debian: sudo apt install tesseract-ocr\n";
}

if (!$onnx_available) {
    echo "\nTo enable embeddings, install ONNX Runtime:\n";
    echo "  macOS: brew install onnxruntime\n";
    echo "  Ubuntu/Debian: sudo apt install libonnxruntime\n";
}
```

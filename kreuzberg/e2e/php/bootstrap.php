<?php

declare(strict_types=1);

/**
 * PHPUnit bootstrap file for E2E tests.
 *
 * This file is loaded before running the test suite and performs the following:
 * - Loads the Composer autoloader for test dependencies
 * - Verifies the Kreuzberg PHP extension is loaded
 * - Sets up the test environment
 */

$autoloadPaths = [
    __DIR__ . '/../../packages/php/vendor/autoload.php',
    __DIR__ . '/vendor/autoload.php',
];

$autoloaded = false;
foreach ($autoloadPaths as $path) {
    if (file_exists($path)) {
        require_once $path;
        $autoloaded = true;
        break;
    }
}

if (!$autoloaded) {
    fwrite(
        STDERR,
        "Error: Could not find Composer autoloader.\n" .
        "Please run 'composer install' in the packages/php directory.\n"
    );
    exit(1);
}

// Register autoloader for E2EPhp namespace tests
spl_autoload_register(function ($class): bool {
    if (strpos($class, 'E2EPhp\\') === 0) {
        $classPath = str_replace('\\', '/', substr($class, 8)); // Remove 'E2EPhp\' prefix
        $file = __DIR__ . '/tests/' . $classPath . '.php';
        if (file_exists($file)) {
            require_once $file;
            return true;
        }
    }
    return false;
}, true, true); // Append to autoload stack, convert to lowercase classname

// Load the Helpers class manually to ensure it's available
require_once __DIR__ . '/tests/Helpers.php';

// Load the mock extension if the real extension is not available
// The extension can be named 'kreuzberg' or 'kreuzberg-php' depending on build configuration
if (!extension_loaded('kreuzberg') && !extension_loaded('kreuzberg-php')) {
    require_once __DIR__ . '/../../packages/php/src/KreuzbergExtensionMock.php';
}

$workspaceRoot = realpath(__DIR__ . '/../../..');
$testDocuments = $workspaceRoot . '/test_documents';

if (!is_dir($testDocuments)) {
    fwrite(
        STDERR,
        "Error: test_documents directory not found at: {$testDocuments}\n" .
        "Please ensure the test_documents directory exists in the workspace root.\n"
    );
    exit(1);
}

echo "PHPUnit E2E Test Suite Bootstrap\n";
echo "=================================\n";
echo "Workspace Root: {$workspaceRoot}\n";
echo "Test Documents: {$testDocuments}\n";
echo "Kreuzberg Extension: Loaded\n";
echo "\n";

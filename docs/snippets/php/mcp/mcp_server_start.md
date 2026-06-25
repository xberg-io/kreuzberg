```php title="PHP"
<?php

declare(strict_types=1);

use Symfony\Component\Process\Process;

// Start the Xberg MCP server as a background process
$process = new Process(['xberg', 'mcp']);
$process->start();

$pid = $process->getPid();
echo "MCP server started with PID: {$pid}\n";

// Wait for server to initialize
sleep(1);

if ($process->isRunning()) {
    echo "Server is running, listening for connections\n";
    echo "Server output: " . $process->getOutput() . "\n";
} else {
    echo "Server failed to start\n";
    echo "Error: " . $process->getErrorOutput() . "\n";
}

// Keep process running or register shutdown handler
register_shutdown_function(function () use ($process): void {
    if ($process->isRunning()) {
        $process->stop();
        echo "MCP server stopped\n";
    }
});
```

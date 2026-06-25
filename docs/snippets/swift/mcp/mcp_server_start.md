<!-- snippet:syntax-only -->

```swift title="Swift"
import Foundation

// Start the xberg MCP server as a subprocess.
// The Swift bindings do not expose an in-process MCP server; use the
// xberg CLI binary which provides the MCP transport over stdio.
let process = Process()
process.executableURL = URL(fileURLWithPath: "/usr/bin/env")
process.arguments = ["xberg", "mcp"]

try process.run()
process.waitUntilExit()
```

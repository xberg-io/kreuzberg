```go title="Document Structure Config (Go)"
package main

import (
    "fmt"
    kreuzberg "github.com/kreuzberg-dev/kreuzberg/packages/go/v5"
)

func main() {
    config := kreuzberg.NewExtractionConfig(
        kreuzberg.WithIncludeDocumentStructure(true),
    )

    result, err := kreuzberg.ExtractFileSync("document.pdf", config)
    if err != nil {
        panic(err)
    }

    if result.Document != nil {
        for _, node := range result.Document.Nodes {
            fmt.Printf("[%s]\n", node.Content.NodeType)
        }
    }
}
```

```go title="Document Structure Config (Go)"
package main

import (
    "fmt"
    xberg "github.com/xberg-io/xberg/packages/go"
)

func main() {
    config := xberg.NewExtractionConfig(
        xberg.WithIncludeDocumentStructure(true),
    )

    input := xberg.ExtractInputFromURI("document.pdf")
    result, err := xberg.Extract(*input, *config)
    if err != nil {
        panic(err)
    }

    if result.Results[0].Document != nil {
        for _, node := range result.Results[0].Document.Nodes {
            fmt.Printf("[%s]\n", node.Content.NodeType)
        }
    }
}
```

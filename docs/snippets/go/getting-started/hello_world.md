```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	result, err := xberg.ExtractFileSync("document.pdf", nil)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	fmt.Println("Extracted content:")
	if len(result.Content) > 200 {
		fmt.Println(result.Content[:200])
	} else {
		fmt.Println(result.Content)
	}
}
```

```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/xberg/packages/go"
)

func main() {
	outputFormat := xberg.OutputFormatHTML
	theme := xberg.HTMLThemeGitHub
	embedCSS := true

	cfg := xberg.ExtractionConfig{
		OutputFormat: &outputFormat,
		HTMLOutput: &xberg.HTMLOutputConfig{
			Theme:    &theme,
			EmbedCSS: &embedCSS,
		},
	}

	input := xberg.ExtractInputFromURI("document.pdf")
	result, err := xberg.Extract(*input, cfg)
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	fmt.Println(result.Results[0].Content) // HTML with kb-* classes
}
```

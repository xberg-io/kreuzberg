```go title="Go"
package main

import (
	"github.com/xberg-io/xberg/packages/go/v5"
)

func main() {
	preserveMarkdown := true
	preserveCode := true
	mode := "moderate"
	languageHint := "eng"

	config := &xberg.ExtractionConfig{
		TokenReduction: &xberg.TokenReductionConfig{
			Mode:             &mode,
			PreserveMarkdown: &preserveMarkdown,
			PreserveCode:     &preserveCode,
			LanguageHint:     &languageHint,
		},
	}
	_ = config
}
```

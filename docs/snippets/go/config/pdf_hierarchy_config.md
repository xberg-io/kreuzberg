```go title="Go"
package main

import "github.com/kreuzberg-dev/kreuzberg/packages/go/v5"

func main() {
	// Basic hierarchy configuration
	config := &kreuzberg.ExtractionConfig{
		PdfOptions: &kreuzberg.PdfConfig{
			ExtractImages: true,
			Hierarchy: &kreuzberg.HierarchyConfig{
				Enabled:               kreuzberg.BoolPtr(true),
				KClusters:             kreuzberg.IntPtr(6),
				IncludeBbox:           kreuzberg.BoolPtr(true),
				OcrCoverageThreshold:  kreuzberg.Float64Ptr(0.8),
			},
		},
	}

	// Advanced hierarchy configuration with more clusters
	advancedConfig := &kreuzberg.ExtractionConfig{
		PdfOptions: &kreuzberg.PdfConfig{
			ExtractImages: true,
			Hierarchy: &kreuzberg.HierarchyConfig{
				Enabled:               kreuzberg.BoolPtr(true),
				KClusters:             kreuzberg.IntPtr(12),
				IncludeBbox:           kreuzberg.BoolPtr(true),
				OcrCoverageThreshold:  kreuzberg.Float64Ptr(0.8),
			},
		},
	}

	_ = config
	_ = advancedConfig
}
```

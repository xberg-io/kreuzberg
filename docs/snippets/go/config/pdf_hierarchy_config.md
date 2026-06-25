```go title="Go"
package main

import "github.com/xberg-io/xberg/packages/go/v5"

func main() {
	enabled := true
	includeBbox := true
	kClusters := uint(6)
	kClustersAdvanced := uint(12)
	threshold := float32(0.8)

	// Basic hierarchy configuration
	config := xberg.ExtractionConfig{
		PdfOptions: &xberg.PdfConfig{
			ExtractImages: true,
			Hierarchy: &xberg.HierarchyConfig{
				Enabled:              &enabled,
				KClusters:            &kClusters,
				IncludeBbox:          &includeBbox,
				OcrCoverageThreshold: &threshold,
			},
		},
	}

	// Advanced hierarchy configuration with more clusters
	advancedConfig := xberg.ExtractionConfig{
		PdfOptions: &xberg.PdfConfig{
			ExtractImages: true,
			Hierarchy: &xberg.HierarchyConfig{
				Enabled:              &enabled,
				KClusters:            &kClustersAdvanced,
				IncludeBbox:          &includeBbox,
				OcrCoverageThreshold: &threshold,
			},
		},
	}

	_ = config
	_ = advancedConfig
}
```

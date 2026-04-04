package kreuzberg

import (
	"testing"
)

// TestPageConfigJSONMarshaling tests JSON serialization of PageConfig.
func TestPageConfigJSONMarshaling(t *testing.T) {
	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages:      BoolPtr(true),
			InsertPageMarkers: BoolPtr(true),
			MarkerFormat:      StringPtr("### Page {page_num} ###"),
		},
	}

	// This tests that the config can be properly serialized
	// The actual FFI call validates JSON encoding
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	_, err = ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("Config marshaling failed: %v", err)
	}
}

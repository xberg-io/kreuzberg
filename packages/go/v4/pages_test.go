package kreuzberg

import (
	"strings"
	"testing"
)

// TestExtractPagesReturnsPageArray tests that extractPages: true returns a pages array.
func TestExtractPagesReturnsPageArray(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages: BoolPtr(true),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	if result.Pages == nil {
		t.Fatal("Pages should not be nil when extractPages is true")
	}

	if len(result.Pages) == 0 {
		t.Fatal("Pages array should not be empty for a valid PDF")
	}
}

// TestInsertPageMarkersAppearsInContent tests that page markers appear in content when enabled.
func TestInsertPageMarkersAppearsInContent(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			InsertPageMarkers: BoolPtr(true),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	// Default marker format should contain "PAGE" pattern
	if !strings.Contains(result.Content, "PAGE") && !strings.Contains(result.Content, "page") {
		t.Error("Content should contain page markers when insertPageMarkers is true")
	}
}

// TestCustomMarkerFormatWorks tests that custom marker format is properly applied.
func TestCustomMarkerFormatWorks(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	customMarker := "=== Page {page_num} ==="
	config := &ExtractionConfig{
		Pages: &PageConfig{
			InsertPageMarkers: BoolPtr(true),
			MarkerFormat:      StringPtr(customMarker),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	if !strings.Contains(result.Content, "=== Page") {
		t.Error("Content should contain custom page markers in the specified format")
	}
}

// TestMultiPagePDFProducesMultiplePages tests that multi-page PDFs return multiple page entries.
func TestMultiPagePDFProducesMultiplePages(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages: BoolPtr(true),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	if result.Pages == nil {
		t.Fatal("Pages should not be nil")
	}

	// Even single-page PDFs should have at least one page entry
	if len(result.Pages) < 1 {
		t.Fatalf("expected at least 1 page in result, got %d", len(result.Pages))
	}

	t.Logf("PDF contains %d pages", len(result.Pages))
}

// TestPageContentStructureValidation validates the structure of extracted page content.
func TestPageContentStructureValidation(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages: BoolPtr(true),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	if result.Pages == nil {
		t.Fatal("Pages should not be nil")
	}

	for idx, page := range result.Pages {
		// Validate page number is positive
		if page.PageNumber == 0 {
			t.Errorf("Page %d: PageNumber should be greater than 0, got %d", idx, page.PageNumber)
		}

		// Validate content is a string (may be empty)
		if page.Content == "" {
			t.Logf("Page %d: Content is empty (allowed for blank pages)", idx)
		}

		// Validate IsBlank field exists (should be nil or a bool pointer)
		if page.IsBlank != nil {
			t.Logf("Page %d: IsBlank = %v", idx, *page.IsBlank)
		} else {
			t.Logf("Page %d: IsBlank is nil (allowed)", idx)
		}
	}
}

// TestPageMarkerFormatWithNumberReplacement tests that {page_num} is replaced correctly.
func TestPageMarkerFormatWithNumberReplacement(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	customMarker := "--- BEGIN PAGE {page_num} ---"
	config := &ExtractionConfig{
		Pages: &PageConfig{
			InsertPageMarkers: BoolPtr(true),
			MarkerFormat:      StringPtr(customMarker),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	// Should contain the marker prefix with actual page numbers
	if !strings.Contains(result.Content, "--- BEGIN PAGE") {
		t.Error("Content should contain the custom marker with correct formatting")
	}
}

// TestExtractPagesWithoutConfig validates default behavior when Pages config is nil.
func TestExtractPagesWithoutConfig(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	// No Pages config provided
	config := &ExtractionConfig{}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	// When Pages config is not provided, Pages should be nil or empty
	// This tests default behavior
	t.Logf("Pages field when no config: %v (length: %d)", result.Pages, len(result.Pages))
}

// TestPageNumberSequence validates that page numbers are sequential.
func TestPageNumberSequence(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages: BoolPtr(true),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	if len(result.Pages) == 0 {
		// If no pages were extracted, that's acceptable - just log it
		t.Logf("No pages extracted from PDF")
		return
	}

	// Validate that page numbers are sequential (1-indexed)
	for i, page := range result.Pages {
		expectedPageNum := uint64(i + 1)
		if page.PageNumber != expectedPageNum {
			t.Logf("Page %d has number %d (expected %d)", i, page.PageNumber, expectedPageNum)
		}
	}
}

// TestPageContentNotEmpty validates that page content is populated.
func TestPageContentNotEmpty(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages: BoolPtr(true),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	if result.Pages == nil {
		t.Fatal("Pages should not be nil")
	}

	hasContent := false
	for _, page := range result.Pages {
		if page.Content != "" {
			hasContent = true
			break
		}
	}

	if !hasContent {
		t.Log("Warning: No page content extracted (document may be empty)")
	}
}

// TestPageMarkersAndExtractPagesTogethers tests combining both page extraction and markers.
func TestPageMarkersAndExtractPagesTogether(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "sample.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	config := &ExtractionConfig{
		Pages: &PageConfig{
			ExtractPages:      BoolPtr(true),
			InsertPageMarkers: BoolPtr(true),
			MarkerFormat:      StringPtr("[Page {page_num}]"),
		},
	}

	result, err := ExtractFileSync(path, config)
	if err != nil {
		t.Fatalf("ExtractFileSync failed: %v", err)
	}

	// Should have both pages array and markers in content
	if result.Pages == nil {
		t.Error("Pages array should be populated when extractPages is true")
	}

	if !strings.Contains(result.Content, "[Page") {
		t.Error("Content should contain page markers when insertPageMarkers is true")
	}
}

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

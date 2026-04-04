// Hand-written binding-specific edge case tests for PDF rendering.
// Happy-path render tests are auto-generated from fixtures in e2e/.
// These tests cover error handling, validation, and lifecycle patterns
// that vary per language and can't be generated uniformly.

package kreuzberg

import (
	"testing"
)

func TestRenderingFunctionsExist(t *testing.T) {
	// Compilation itself proves the functions exist
	_ = RenderPdfPage
	_ = NewPdfPageIterator
}

func TestRenderPdfPageNonexistentFile(t *testing.T) {
	_, err := RenderPdfPage("/nonexistent/path/to/document.pdf", 0, 150)
	if err == nil {
		t.Fatal("expected error for nonexistent file")
	}
}

func TestRenderPdfPageEmptyPath(t *testing.T) {
	_, err := RenderPdfPage("", 0, 150)
	if err == nil {
		t.Fatal("expected error for empty path")
	}
}

func TestRenderPdfPageIndexOutOfBounds(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "test.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	_, err = RenderPdfPage(path, 9999, 150)
	if err == nil {
		t.Fatal("expected error for out-of-bounds page index")
	}
}

func TestRenderPdfPageNegativeIndex(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "test.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	_, err = RenderPdfPage(path, -1, 150)
	if err == nil {
		t.Fatal("expected error for negative page index")
	}
}

func TestPdfPageIteratorClose(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "test.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	iter, err := NewPdfPageIterator(path, 150)
	if err != nil {
		t.Fatalf("NewPdfPageIterator failed: %v", err)
	}

	iter.Close()
	// Double-close should be safe
	iter.Close()

	// After close, PageCount returns 0
	if pc := iter.PageCount(); pc != 0 {
		t.Fatalf("expected PageCount 0 after close, got %d", pc)
	}

	// After close, Next returns ok=false
	_, _, ok, err := iter.Next()
	if err != nil {
		t.Fatalf("Next after close returned error: %v", err)
	}
	if ok {
		t.Fatal("Next after close should return ok=false")
	}
}

func TestPdfPageIteratorNonexistentFile(t *testing.T) {
	_, err := NewPdfPageIterator("/nonexistent/path/to/document.pdf", 150)
	if err == nil {
		t.Fatal("expected error for nonexistent file")
	}
}

func TestPdfPageIteratorEmptyPath(t *testing.T) {
	_, err := NewPdfPageIterator("", 150)
	if err == nil {
		t.Fatal("expected error for empty path")
	}
}

func TestPdfPageIteratorEarlyTermination(t *testing.T) {
	dir := t.TempDir()
	path, err := writeValidPDFToFile(dir, "test.pdf")
	if err != nil {
		t.Fatalf("failed to write test PDF: %v", err)
	}

	iter, err := NewPdfPageIterator(path, 150)
	if err != nil {
		t.Fatalf("NewPdfPageIterator failed: %v", err)
	}
	defer iter.Close()

	// Read one page, then stop
	pageIndex, png, ok, err := iter.Next()
	if err != nil {
		t.Fatalf("Next returned error: %v", err)
	}
	if !ok {
		t.Fatal("expected at least one page")
	}
	if pageIndex != 0 {
		t.Fatalf("expected page index 0, got %d", pageIndex)
	}
	// Verify it's valid PNG (starts with PNG magic bytes)
	if len(png) < 8 {
		t.Fatal("PNG data too short")
	}
	pngMagic := []byte{0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A}
	for i, b := range pngMagic {
		if png[i] != b {
			t.Fatalf("invalid PNG magic byte at offset %d: got %x, want %x", i, png[i], b)
		}
	}
	// Close without exhausting the iterator
}

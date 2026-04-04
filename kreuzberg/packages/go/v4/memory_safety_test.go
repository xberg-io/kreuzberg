package kreuzberg_test

import (
	"context"
	"errors"
	"runtime"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v4"
)

// TestPointerValidityAfterFFICall verifies that pointers remain valid after FFI calls
// and that the conversion from C structures to Go structures preserves data integrity.
func TestPointerValidityAfterFFICall(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// First call to establish baseline
	result1, err := kreuzberg.ExtractFileSync(testPDF, nil)
	if err != nil {
		t.Fatalf("first extraction failed: %v", err)
	}
	if result1 == nil {
		t.Fatal("expected non-nil result")
	}

	// Second call with same file - should not affect first result
	result2, err := kreuzberg.ExtractFileSync(testPDF, nil)
	if err != nil {
		t.Fatalf("second extraction failed: %v", err)
	}
	if result2 == nil {
		t.Fatal("expected non-nil result")
	}

	// Verify both results are still valid and have same content
	if result1.Content != result2.Content {
		t.Errorf("content mismatch after second FFI call: got %q, expected %q",
			result1.Content, result2.Content)
	}
	if result1.MimeType != result2.MimeType {
		t.Errorf("mime type mismatch: got %q, expected %q",
			result1.MimeType, result2.MimeType)
	}
}

// TestGoroutineCountStability verifies that extraction operations do not spawn
// persistent goroutines that are never cleaned up, indicating potential leaks.
func TestGoroutineCountStability(t *testing.T) {

	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Wait for any background goroutines to settle
	// Use a loop to let runtime stabilize
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(10 * time.Millisecond)
	}

	initialGoroutines := runtime.NumGoroutine()

	// Perform multiple extraction operations
	for i := 0; i < 5; i++ {
		result, err := kreuzberg.ExtractFileSync(testPDF, nil)
		if err != nil {
			t.Fatalf("extraction %d failed: %v", i, err)
		}
		if result == nil {
			t.Fatalf("extraction %d returned nil result", i)
		}
		// Force result to be unused - go compiler won't optimize it away
		_ = result.Content
	}

	// Wait for operations to fully complete and be cleaned up
	// Use loop instead of single sleep for reliability
	for i := 0; i < 20; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Should not spawn persistent goroutines (allow 1 for timing variance)
	if leakedGoroutines > 1 {
		t.Errorf("goroutine leak detected: initial=%d, final=%d, leaked=%d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

// TestGoroutineCleanupOnPanic verifies that resources are cleaned up when
// extraction operations are called from goroutines that panic.
func TestGoroutineCleanupOnPanic(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Establish baseline - settle runtime first
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	var wg sync.WaitGroup
	var panicCount int32
	wg.Add(5)

	for i := 0; i < 5; i++ {
		go func(index int) {
			defer wg.Done()
			defer func() {
				if r := recover(); r != nil {
					atomic.AddInt32(&panicCount, 1)
				}
			}()

			result, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				t.Errorf("extraction failed: %v", err)
				return
			}

			// Verify result is valid
			if result == nil {
				panic("nil result")
			}

			// Intentional panic to test cleanup
			if index == 2 {
				panic("test panic")
			}
		}(i)
	}

	wg.Wait()

	// Wait for goroutines to fully exit (multiple cycles for safety)
	for i := 0; i < 20; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Verify panic handler was executed
	if atomic.LoadInt32(&panicCount) == 0 {
		t.Error("expected at least one panic to be caught")
	}

	// Should not have any persistent goroutines (allow 0)
	if leakedGoroutines > 0 {
		t.Errorf("goroutine leak detected: initial: %d, final: %d, leaked: %d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

// TestConcurrentGoroutineLeakDetection verifies that concurrent extractions
// do not spawn leaking goroutines by comparing goroutine counts.
func TestConcurrentGoroutineLeakDetection(t *testing.T) {

	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Establish stable baseline
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	// Perform many concurrent operations
	const operations = 100
	var wg sync.WaitGroup
	wg.Add(operations)

	for i := 0; i < operations; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				t.Logf("extraction %d failed: %v", index, err)
				return
			}
			if result == nil {
				t.Logf("extraction %d returned nil", index)
			}
		}(i)
	}

	wg.Wait()

	// Wait for all goroutines to exit
	for i := 0; i < 30; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Should not spawn persistent goroutines from operations
	if leakedGoroutines > 1 {
		t.Errorf("goroutine leak in concurrent operations: initial=%d, final=%d, leaked=%d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

// TestMultipleSequentialExtractions verifies that back-to-back extractions
// work correctly without state pollution between operations.
func TestMultipleSequentialExtractions(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Establish baseline goroutine count
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	const iterations = 50
	lastResult := (*kreuzberg.ExtractionResult)(nil)

	for i := 0; i < iterations; i++ {
		result, err := kreuzberg.ExtractFileSync(testPDF, nil)
		if err != nil {
			t.Fatalf("iteration %d: extraction failed: %v", i, err)
		}
		if result == nil {
			t.Fatalf("iteration %d: expected non-nil result", i)
		}
		if result.MimeType == "" {
			t.Errorf("iteration %d: expected non-empty mime type", i)
		}

		// Verify consistency across iterations
		if lastResult != nil && result.MimeType != lastResult.MimeType {
			t.Errorf("iteration %d: mime type changed from %q to %q",
				i, lastResult.MimeType, result.MimeType)
		}
		lastResult = result
	}

	// Wait for cleanup
	for i := 0; i < 20; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Should not leak goroutines in sequential operations
	if leakedGoroutines > 1 {
		t.Errorf("goroutine leak in sequential operations: initial=%d, final=%d, leaked=%d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

// TestStringMarshalingSafety verifies that result serialization is correct
// and reproducible across multiple extraction operations.
func TestStringMarshalingSafety(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	const iterations = 10
	var firstJSON string

	// Extract multiple times and verify JSON is consistent
	for i := 0; i < iterations; i++ {
		result, err := kreuzberg.ExtractFileSync(testPDF, nil)
		if err != nil {
			t.Fatalf("iteration %d: extraction failed: %v", i, err)
		}

		if result == nil {
			t.Fatalf("iteration %d: expected non-nil result", i)
		}

		// Verify mime type is consistent
		if result.MimeType == "" {
			t.Errorf("iteration %d: mime type should not be empty", i)
		}

		// Verify metadata pointers don't reference empty strings
		if result.Metadata.Language != nil && *result.Metadata.Language == "" {
			t.Errorf("iteration %d: empty language pointer detected", i)
		}

		// Verify JSON marshaling is reproducible
		jsonBytes, err := kreuzberg.ResultToJSON(result)
		if err != nil {
			t.Fatalf("iteration %d: JSON marshaling failed: %v", i, err)
		}

		currentJSON := string(jsonBytes)
		if i == 0 {
			firstJSON = currentJSON
		} else if currentJSON != firstJSON {
			t.Errorf("iteration %d: JSON output differs from first extraction", i)
		}
	}
}

// TestConcurrentResultReads verifies that reading the same result object
// from multiple goroutines is safe (designed for -race flag verification).
func TestConcurrentResultReads(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	result, err := kreuzberg.ExtractFileSync(testPDF, nil)
	if err != nil {
		t.Fatalf("extraction failed: %v", err)
	}
	if result == nil {
		t.Fatal("expected non-nil result")
	}

	// Run concurrent reads on the same result
	// This test is best run with: go test -race ./...
	const numGoroutines = 10
	var wg sync.WaitGroup
	var readCount int32
	wg.Add(numGoroutines)

	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()

			// Read all fields concurrently
			content := result.Content
			mimeType := result.MimeType

			// Verify data consistency
			if mimeType == "" {
				t.Errorf("goroutine %d: empty mime type", index)
			}
			_ = content // Use content to avoid unused variable

			// Attempt JSON marshaling concurrently
			_, err := kreuzberg.ResultToJSON(result)
			if err != nil {
				t.Errorf("goroutine %d: marshaling failed: %v", index, err)
			}

			atomic.AddInt32(&readCount, 1)

			// Prevent compiler optimizations
			_ = content
		}(i)
	}

	wg.Wait()

	// Verify all goroutines executed
	if atomic.LoadInt32(&readCount) != numGoroutines {
		t.Errorf("expected %d reads, got %d", numGoroutines, readCount)
	}
}

// TestBatchOperationGoroutineCleanup verifies that batch extraction operations
// do not leak goroutines, using goroutine count as a proxy for resource leaks.
func TestBatchOperationGoroutineCleanup(t *testing.T) {

	// Establish baseline goroutine count
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	// Batch operation that allocates many C structures
	pdfBytes := generateTestPDFBytes(t)
	items := make([]kreuzberg.BytesWithMime, 10)
	for i := 0; i < 10; i++ {
		items[i] = kreuzberg.BytesWithMime{
			Data:     pdfBytes,
			MimeType: "application/pdf",
		}
	}

	results, err := kreuzberg.BatchExtractBytesSync(items, nil)
	if err != nil {
		t.Logf("batch extraction error: %v", err)
	}

	if len(results) == 0 {
		t.Log("batch returned no results")
	} else if len(results) != len(items) {
		t.Logf("batch returned %d results for %d items (expected %d)", len(results), len(items), len(items))
	}

	// Wait for cleanup cycles
	for i := 0; i < 20; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Should not spawn persistent goroutines from batch operation
	if leakedGoroutines > 1 {
		t.Errorf("goroutine leak in batch operation: initial=%d, final=%d, leaked=%d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

// TestErrorPathCleanup verifies that failed operations don't leak goroutines
// and resources are properly cleaned up after error conditions.
func TestErrorPathCleanup(t *testing.T) {
	// Establish baseline goroutine count
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	// Intentionally cause errors to verify cleanup
	const errorIterations = 30
	for i := 0; i < errorIterations; i++ {
		// Invalid file path should trigger error path
		_, err1 := kreuzberg.ExtractFileSync("/nonexistent/invalid/path/file.pdf", nil)
		if err1 == nil {
			t.Error("expected error for nonexistent file")
		}

		// Batch with empty path should trigger error path
		_, err3 := kreuzberg.BatchExtractFilesSync([]string{""}, nil)
		if err3 == nil {
			t.Error("expected error for batch with empty path")
		}
	}

	// Wait for cleanup (more time on ARM64 due to slower GC)
	cleanupIterations := 20
	if runtime.GOARCH == "arm64" {
		cleanupIterations = 40
	}
	for i := 0; i < cleanupIterations; i++ {
		runtime.GC()
		time.Sleep(10 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Allow more tolerance on ARM64 due to platform-specific GC behavior
	maxLeaked := 1
	if runtime.GOARCH == "arm64" {
		maxLeaked = 3
	}

	if leakedGoroutines > maxLeaked {
		t.Errorf("goroutine leak in error paths: initial=%d, final=%d, leaked=%d (max allowed: %d)",
			initialGoroutines, finalGoroutines, leakedGoroutines, maxLeaked)
	}
}

// TestContextCancellationCleanup ensures proper cleanup and goroutine handling
// when context is canceled before or during FFI operations.
func TestContextCancellationCleanup(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Establish baseline
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	// Create pre-canceled context and attempt many operations
	const iterations = 20
	var cancelledCount int32
	for i := 0; i < iterations; i++ {
		ctx, cancel := context.WithCancel(context.Background())
		cancel() // Cancel immediately

		_, err := kreuzberg.ExtractFileWithContext(ctx, testPDF, nil)
		if errors.Is(err, context.Canceled) {
			atomic.AddInt32(&cancelledCount, 1)
		}
	}

	// Wait for cleanup
	for i := 0; i < 20; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	// Verify cancellation was respected
	if atomic.LoadInt32(&cancelledCount) == 0 {
		t.Error("expected context cancellation to be detected")
	}

	// Should not leak goroutines from canceled operations
	if leakedGoroutines > 1 {
		t.Errorf("goroutine leak in canceled operations: initial=%d, final=%d, leaked=%d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

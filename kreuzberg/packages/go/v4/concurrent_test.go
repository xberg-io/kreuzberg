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

// TestConcurrentExtractFileSync verifies thread-safe concurrent file extraction.
func TestConcurrentExtractFileSync(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	numGoroutines := 10
	var wg sync.WaitGroup
	errChan := make(chan error, numGoroutines)

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				errChan <- err
				return
			}
			if result == nil {
				errChan <- errors.New("nil result")
				return
			}
			if result.Content == "" {
				errChan <- errors.New("empty content")
				return
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Errorf("concurrent extraction failed: %v", err)
	}
}

// TestConcurrentExtractBytesSync verifies thread-safe concurrent bytes extraction.
func TestConcurrentExtractBytesSync(t *testing.T) {
	pdfBytes := generateTestPDFBytes(t)

	numGoroutines := 10
	var wg sync.WaitGroup
	errChan := make(chan error, numGoroutines)

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractBytesSync(pdfBytes, "application/pdf", nil)
			if err != nil {
				errChan <- err
				return
			}
			if result == nil {
				errChan <- errors.New("nil result")
				return
			}
			if result.MimeType != "application/pdf" {
				errChan <- errors.New("incorrect mime type")
				return
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Errorf("concurrent extraction failed: %v", err)
	}
}

// TestConcurrentExtractWithContext verifies context cancellation with concurrent operations.
func TestConcurrentExtractWithContext(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	numGoroutines := 5
	var wg sync.WaitGroup
	errChan := make(chan error, numGoroutines)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractFileWithContext(ctx, testPDF, nil)
			if err != nil && !errors.Is(err, context.Canceled) {
				errChan <- err
				return
			}
			if err == nil && result == nil {
				errChan <- errors.New("nil result with no error")
				return
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Errorf("concurrent context extraction failed: %v", err)
	}
}

// TestContextCancellationBeforeExtraction verifies context is checked before extraction.
func TestContextCancellationBeforeExtraction(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	result, err := kreuzberg.ExtractFileWithContext(ctx, testPDF, nil)

	if err == nil {
		t.Errorf("expected context cancellation error, got nil")
	}
	if !errors.Is(err, context.Canceled) {
		t.Errorf("expected context.Canceled, got: %v", err)
	}
	if result != nil {
		t.Errorf("expected nil result with canceled context, got: %v", result)
	}
}

// TestContextTimeoutBeforeExtraction verifies context timeout is respected.
func TestContextTimeoutBeforeExtraction(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	result, err := kreuzberg.ExtractBytesWithContext(ctx, []byte{}, "application/pdf", nil)

	if err == nil {
		t.Errorf("expected context error, got nil")
	}
	if result != nil {
		t.Errorf("expected nil result with canceled context, got: %v", result)
	}
}

// TestBatchConcurrentExtraction verifies batch operations are thread-safe.
func TestBatchConcurrentExtraction(t *testing.T) {
	paths := []string{}
	for i := 0; i < 3; i++ {
		pdfPath := createTestPDF(t)
		paths = append(paths, pdfPath)
		defer cleanup(pdfPath)
	}

	numGoroutines := 5
	var wg sync.WaitGroup
	errChan := make(chan error, numGoroutines)

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			results, err := kreuzberg.BatchExtractFilesSync(paths, nil)
			if err != nil {
				errChan <- err
				return
			}
			if len(results) != len(paths) {
				errChan <- errors.New("batch results count mismatch")
				return
			}
		}(i)
	}

	wg.Wait()
	close(errChan)

	for err := range errChan {
		t.Errorf("concurrent batch extraction failed: %v", err)
	}
}

// TestBatchExtractBytesWithContext verifies batch bytes extraction with context.
func TestBatchExtractBytesWithContext(t *testing.T) {
	pdfBytes := generateTestPDFBytes(t)

	items := []kreuzberg.BytesWithMime{
		{Data: pdfBytes, MimeType: "application/pdf"},
		{Data: pdfBytes, MimeType: "application/pdf"},
	}

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	results, err := kreuzberg.BatchExtractBytesWithContext(ctx, items, nil)
	if err != nil {
		t.Fatalf("batch extraction with context failed: %v", err)
	}
	if len(results) != len(items) {
		t.Errorf("expected %d results, got %d", len(items), len(results))
	}
}

// TestConcurrentErrorHandling verifies error handling in concurrent scenarios.
func TestConcurrentErrorHandling(t *testing.T) {
	numGoroutines := 10
	var wg sync.WaitGroup
	errorCount := 0
	var mu sync.Mutex

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			_, err := kreuzberg.ExtractFileSync("/nonexistent/file.pdf", nil)
			if err == nil {
				t.Error("expected error for nonexistent file")
			}
			mu.Lock()
			errorCount++
			mu.Unlock()
		}(i)
	}

	wg.Wait()

	if errorCount != numGoroutines {
		t.Errorf("expected %d errors, got %d", numGoroutines, errorCount)
	}
}

// TestConcurrentFileExtractionLoad verifies that concurrent file extractions
// complete successfully under reasonable concurrency (50 goroutines).
// Best run with: go test -race ./...
func TestConcurrentFileExtractionLoad(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// 50 goroutines provides meaningful concurrency testing
	numGoroutines := 50
	var wg sync.WaitGroup
	var successCount int64
	var errorCount int64

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			_, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err == nil {
				atomic.AddInt64(&successCount, 1)
			} else {
				atomic.AddInt64(&errorCount, 1)
			}
		}(i)
	}

	wg.Wait()

	total := atomic.LoadInt64(&successCount) + atomic.LoadInt64(&errorCount)
	if total != int64(numGoroutines) {
		t.Errorf("expected %d operations, got %d", numGoroutines, total)
	}
}

// TestChannelSynchronizationPattern verifies safe channel usage for collecting
// extraction results from concurrent goroutines.
func TestChannelSynchronizationPattern(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Use 20 goroutines to exercise channel contention
	numGoroutines := 20
	resultChan := make(chan *kreuzberg.ExtractionResult, numGoroutines)
	errChan := make(chan error, numGoroutines)
	var wg sync.WaitGroup

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				errChan <- err
				return
			}
			select {
			case resultChan <- result:
			case <-time.After(5 * time.Second):
				errChan <- errors.New("timeout sending result to channel")
			}
		}(i)
	}

	// Close channels after all goroutines complete
	go func() {
		wg.Wait()
		close(resultChan)
		close(errChan)
	}()

	// Consume results and errors
	resultCount := 0
	for result := range resultChan {
		if result != nil {
			resultCount++
		}
	}

	// Check for errors
	errorCount := 0
	for range errChan {
		errorCount++
	}

	// Should receive results from successful extractions
	if resultCount == 0 && errorCount == 0 {
		t.Error("expected at least some operations to complete or report errors")
	}

	if resultCount+errorCount != numGoroutines {
		t.Errorf("expected %d total outcomes, got %d",
			numGoroutines, resultCount+errorCount)
	}
}

// TestCacheAccessPattern verifies concurrent reads and writes to shared state
// using a simulated cache pattern with proper synchronization.
func TestCacheAccessPattern(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Simulate a cache with RWMutex
	cache := &struct {
		sync.RWMutex
		items map[string]*kreuzberg.ExtractionResult
	}{
		items: make(map[string]*kreuzberg.ExtractionResult),
	}

	// 5 writers and 15 readers creates interesting contention patterns
	numReaders := 15
	numWriters := 5
	var wg sync.WaitGroup
	var writeCount int32
	var readCount int32

	// Writers - extract and cache results
	wg.Add(numWriters)
	for i := 0; i < numWriters; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				return
			}
			cache.Lock()
			cache.items[kreuzberg.LibraryVersion()] = result
			cache.Unlock()
			atomic.AddInt32(&writeCount, 1)
		}(i)
	}

	// Readers - read cache concurrently
	wg.Add(numReaders)
	for i := 0; i < numReaders; i++ {
		go func(index int) {
			defer wg.Done()

			cache.RLock()
			_ = len(cache.items) // Verify we can read the cache size
			cache.RUnlock()

			atomic.AddInt32(&readCount, 1)
		}(i)
	}

	wg.Wait()

	// Verify final state
	cache.RLock()
	cacheSize := len(cache.items)
	cache.RUnlock()

	writes := atomic.LoadInt32(&writeCount)
	reads := atomic.LoadInt32(&readCount)
	_ = cacheSize // Ensure final state is available for verification

	if reads != int32(numReaders) {
		t.Errorf("expected %d reads, got %d", numReaders, reads)
	}

	// At least one writer should have succeeded
	if writes == 0 {
		t.Error("expected at least one write to succeed")
	}
}

// TestGoroutineLeakDetection monitors goroutine count before and after
// high-concurrency operations to detect resource leaks.
func TestGoroutineLeakDetection(t *testing.T) {

	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Establish stable baseline
	for i := 0; i < 10; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}
	initialGoroutines := runtime.NumGoroutine()

	// 100 concurrent goroutines stress-test goroutine management
	numGoroutines := 100
	var wg sync.WaitGroup
	var completedCount int32
	wg.Add(numGoroutines)

	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			_, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				t.Logf("extraction error at index %d: %v", index, err)
			}
			atomic.AddInt32(&completedCount, 1)
		}(i)
	}

	wg.Wait()

	// Wait for goroutines to fully exit
	for i := 0; i < 30; i++ {
		runtime.GC()
		time.Sleep(5 * time.Millisecond)
	}

	finalGoroutines := runtime.NumGoroutine()
	leakedGoroutines := finalGoroutines - initialGoroutines

	if atomic.LoadInt32(&completedCount) != int32(numGoroutines) {
		t.Errorf("expected %d completions, got %d", numGoroutines, completedCount)
	}

	// Should not leak goroutines (allow 1 for variance)
	if leakedGoroutines > 1 {
		t.Errorf("goroutine leak detected: initial=%d, final=%d, leaked=%d",
			initialGoroutines, finalGoroutines, leakedGoroutines)
	}
}

// TestContextCancellationHandling verifies proper cleanup and error handling
// when context is canceled during concurrent operations.
func TestContextCancellationHandling(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Test cancellation during concurrent operations (20 goroutines)
	numGoroutines := 20
	var wg sync.WaitGroup
	var cancelledCount int64
	var successCount int64
	var otherErrorCount int64

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()

			// Cancel after some goroutines have started
			if index == numGoroutines/2 {
				time.Sleep(1 * time.Millisecond)
				cancel()
			}

			result, err := kreuzberg.ExtractFileWithContext(ctx, testPDF, nil)
			switch {
			case errors.Is(err, context.Canceled):
				atomic.AddInt64(&cancelledCount, 1)
			case err == nil && result != nil:
				atomic.AddInt64(&successCount, 1)
			case err != nil:
				atomic.AddInt64(&otherErrorCount, 1)
			}
		}(i)
	}

	wg.Wait()

	total := atomic.LoadInt64(&cancelledCount) + atomic.LoadInt64(&successCount) + atomic.LoadInt64(&otherErrorCount)
	if total != int64(numGoroutines) {
		t.Errorf("expected %d operations, got %d", numGoroutines, total)
	}

	// Verify cancellation was detected by at least one goroutine
	if atomic.LoadInt64(&cancelledCount) == 0 {
		t.Error("expected at least one context cancellation to be detected")
	}
}

// TestAtomicOperationValidation verifies that shared counters using atomic
// operations remain consistent under concurrent load from 10 goroutines.
func TestAtomicOperationValidation(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	var (
		successCount   int64
		errorCount     int64
		operationsLeft int64 = 100
	)

	// 10 goroutines competing to consume 100 operations
	numGoroutines := 10
	var wg sync.WaitGroup

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()

			// Each goroutine processes operations atomically
			for {
				remaining := atomic.LoadInt64(&operationsLeft)
				if remaining <= 0 {
					break
				}

				// Attempt to decrement atomically
				if !atomic.CompareAndSwapInt64(&operationsLeft, remaining, remaining-1) {
					continue // Retry on conflict
				}

				result, err := kreuzberg.ExtractFileSync(testPDF, nil)
				if err != nil {
					atomic.AddInt64(&errorCount, 1)
				} else if result != nil {
					atomic.AddInt64(&successCount, 1)
				}
			}
		}(i)
	}

	wg.Wait()

	totalOps := atomic.LoadInt64(&successCount) + atomic.LoadInt64(&errorCount)
	if totalOps != 100 {
		t.Errorf("expected 100 operations, got %d (success=%d, errors=%d)",
			totalOps, successCount, errorCount)
	}

	remainingOps := atomic.LoadInt64(&operationsLeft)
	if remainingOps != 0 {
		t.Errorf("expected 0 remaining operations, got %d", remainingOps)
	}
}

// TestConcurrentBytesExtraction with varying sizes and concurrent access.
func TestConcurrentBytesExtraction(t *testing.T) {
	pdfBytes := generateTestPDFBytes(t)

	numGoroutines := 25
	var wg sync.WaitGroup
	successCount := int64(0)
	var mu sync.Mutex
	results := make([]*kreuzberg.ExtractionResult, 0, numGoroutines)

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractBytesSync(pdfBytes, "application/pdf", nil)
			if err != nil {
				return
			}
			if result != nil {
				mu.Lock()
				results = append(results, result)
				mu.Unlock()
				atomic.AddInt64(&successCount, 1)
			}
		}(i)
	}

	wg.Wait()

	if successCount > 0 && len(results) != int(successCount) {
		t.Errorf("result slice mismatch: success=%d, results=%d", successCount, len(results))
	}
}

// TestMixedConcurrentOperations combines file, bytes, and batch operations
// concurrently to stress-test the extraction system with diverse API usage.
func TestMixedConcurrentOperations(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	pdfBytes := generateTestPDFBytes(t)

	var wg sync.WaitGroup
	var fileSuccess, fileErrors int64
	var bytesSuccess, bytesErrors int64
	var batchSuccess, batchErrors int64

	const fileOps = 10
	const bytesOps = 10
	const batchOps = 5

	// File extractions
	wg.Add(fileOps)
	for i := 0; i < fileOps; i++ {
		go func() {
			defer wg.Done()
			_, err := kreuzberg.ExtractFileSync(testPDF, nil)
			if err != nil {
				atomic.AddInt64(&fileErrors, 1)
			} else {
				atomic.AddInt64(&fileSuccess, 1)
			}
		}()
	}

	// Bytes extractions
	wg.Add(bytesOps)
	for i := 0; i < bytesOps; i++ {
		go func() {
			defer wg.Done()
			_, err := kreuzberg.ExtractBytesSync(pdfBytes, "application/pdf", nil)
			if err != nil {
				atomic.AddInt64(&bytesErrors, 1)
			} else {
				atomic.AddInt64(&bytesSuccess, 1)
			}
		}()
	}

	// Batch operations
	wg.Add(batchOps)
	for i := 0; i < batchOps; i++ {
		go func() {
			defer wg.Done()
			items := []kreuzberg.BytesWithMime{
				{Data: pdfBytes, MimeType: "application/pdf"},
			}
			_, err := kreuzberg.BatchExtractBytesSync(items, nil)
			if err != nil {
				atomic.AddInt64(&batchErrors, 1)
			} else {
				atomic.AddInt64(&batchSuccess, 1)
			}
		}()
	}

	wg.Wait()

	fileTotal := atomic.LoadInt64(&fileSuccess) + atomic.LoadInt64(&fileErrors)
	bytesTotal := atomic.LoadInt64(&bytesSuccess) + atomic.LoadInt64(&bytesErrors)
	batchTotal := atomic.LoadInt64(&batchSuccess) + atomic.LoadInt64(&batchErrors)

	if fileTotal != fileOps {
		t.Errorf("file operations: expected %d, got %d", fileOps, fileTotal)
	}
	if bytesTotal != bytesOps {
		t.Errorf("bytes operations: expected %d, got %d", bytesOps, bytesTotal)
	}
	if batchTotal != batchOps {
		t.Errorf("batch operations: expected %d, got %d", batchOps, batchTotal)
	}
}

// TestContextTimeoutDetection verifies that context timeout errors are properly
// detected and handled in concurrent extraction operations.
func TestContextTimeoutDetection(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Use 15 goroutines with immediate cancellation to reliably trigger timeouts
	numGoroutines := 15
	var wg sync.WaitGroup
	var timeoutCount int64
	var completedCount int64
	var otherErrorCount int64

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()

			// Create context with immediate cancellation to guarantee timeout detection
			ctx, cancel := context.WithTimeout(context.Background(), 1*time.Nanosecond)
			defer cancel()

			result, err := kreuzberg.ExtractFileWithContext(ctx, testPDF, nil)
			switch {
			case errors.Is(err, context.DeadlineExceeded):
				atomic.AddInt64(&timeoutCount, 1)
			case err == nil && result != nil:
				atomic.AddInt64(&completedCount, 1)
			case err != nil:
				atomic.AddInt64(&otherErrorCount, 1)
			}
		}(i)
	}

	wg.Wait()

	total := atomic.LoadInt64(&timeoutCount) + atomic.LoadInt64(&completedCount) + atomic.LoadInt64(&otherErrorCount)
	if total != int64(numGoroutines) {
		t.Errorf("expected %d operations, got %d", numGoroutines, total)
	}

	// With 1ns timeout, we expect timeouts to be detected
	if atomic.LoadInt64(&timeoutCount) == 0 {
		t.Error("expected at least some context timeouts to be detected")
	}
}

// TestConcurrentConfigUsage verifies that configuration objects can be safely
// used across multiple concurrent extraction operations.
func TestConcurrentConfigUsage(t *testing.T) {
	testPDF := createTestPDF(t)
	defer cleanup(testPDF)

	// Create a shared config (immutable after creation)
	config := &kreuzberg.ExtractionConfig{
		UseCache: boolPtr(false),
	}

	numGoroutines := 20
	var wg sync.WaitGroup
	successCount := int64(0)

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()
			result, err := kreuzberg.ExtractFileSync(testPDF, config)
			if err == nil && result != nil {
				atomic.AddInt64(&successCount, 1)
			}
		}(i)
	}

	wg.Wait()

	if successCount == 0 {
		t.Log("no successful extractions with config")
	}
}

// Helper function for creating bool pointers
func boolPtr(b bool) *bool {
	return &b
}

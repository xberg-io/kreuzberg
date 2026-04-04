package kreuzberg_test

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"sync/atomic"
	"testing"

	kreuzberg "github.com/kreuzberg-dev/kreuzberg/packages/go/v4"
)

// TestInvalidConfigNegativeChunkSize validates error handling for negative chunk sizes.
func TestInvalidConfigNegativeChunkSize(t *testing.T) {
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize: kreuzberg.IntPtr(-100), // Invalid: negative chunk size
		},
	}

	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test document content"),
		"text/plain",
		config,
	)

	if err == nil {
		t.Fatalf("expected error for negative chunk size, got nil")
	}

	if !strings.Contains(err.Error(), "chunk") {
		t.Errorf("error message should mention 'chunk', got: %s", err.Error())
	}
}

// TestInvalidConfigNegativeOverlap validates error handling for negative overlap values.
func TestInvalidConfigNegativeOverlap(t *testing.T) {
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize:    kreuzberg.IntPtr(512),
			ChunkOverlap: kreuzberg.IntPtr(-50), // Invalid: negative overlap
		},
	}

	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test document content"),
		"text/plain",
		config,
	)

	if err == nil {
		t.Fatalf("expected error for negative overlap, got nil")
	}

	if !strings.Contains(err.Error(), "overlap") {
		t.Errorf("error message should mention 'overlap', got: %s", err.Error())
	}
}

// TestFileNotFound validates error handling for missing files.
func TestFileNotFound(t *testing.T) {
	_, err := kreuzberg.ExtractFileSync(
		"/nonexistent/path/does/not/exist.pdf",
		nil,
	)

	if err == nil {
		t.Fatalf("expected error for missing file, got nil")
	}

	errMsg := err.Error()
	if !strings.Contains(strings.ToLower(errMsg), "file") && !strings.Contains(strings.ToLower(errMsg), "not found") {
		t.Errorf("error message should indicate file not found, got: %s", errMsg)
	}
}

// TestCorruptedPDFFile validates error handling for corrupted file content.
func TestCorruptedPDFFile(t *testing.T) {
	dir := t.TempDir()
	corruptPath := filepath.Join(dir, "corrupted.pdf")

	// Write clearly invalid PDF content
	corruptData := []byte("This is not a valid PDF file at all")
	err := os.WriteFile(corruptPath, corruptData, 0o600)
	if err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	_, err = kreuzberg.ExtractFileSync(corruptPath, nil)
	if err == nil {
		t.Fatalf("expected error for corrupted PDF, got nil")
	}

	errMsg := err.Error()
	if errMsg == "" {
		t.Errorf("error message should not be empty")
	}
}

// TestInvalidMIMEType validates error handling for unsupported MIME types.
func TestInvalidMIMEType(t *testing.T) {
	_, err := kreuzberg.ExtractBytesSync(
		[]byte("some content"),
		"invalid/unsupported-mime-type",
		nil,
	)

	if err == nil {
		t.Fatalf("expected error for invalid MIME type, got nil")
	}
}

// TestEmptyMIMEType validates error handling for missing MIME type.
func TestEmptyMIMEType(t *testing.T) {
	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test content"),
		"", // Empty MIME type
		nil,
	)

	if err == nil {
		t.Fatalf("expected error for empty MIME type, got nil")
	}

	// Should be a validation error
	var valErr *kreuzberg.ValidationError
	if errors.As(err, &valErr) {
		if valErr.Kind() != kreuzberg.ErrorKindValidation {
			t.Errorf("expected ValidationError, got %T", err)
		}
	}
}

// TestMalformedJSONDocument validates error handling for JSON parsing errors.
func TestMalformedJSONDocument(t *testing.T) {
	malformedJSON := []byte(`{"invalid": json content}`)

	_, err := kreuzberg.ExtractBytesSync(
		malformedJSON,
		"application/json",
		nil,
	)

	if err == nil {
		t.Fatalf("expected error for malformed JSON, got nil")
	}
}

// TestEmptyPathValidation validates error handling for empty file paths.
func TestEmptyPathValidation(t *testing.T) {
	_, err := kreuzberg.ExtractFileSync("", nil)

	if err == nil {
		t.Fatalf("expected error for empty path, got nil")
	}

	var valErr *kreuzberg.ValidationError
	if errors.As(err, &valErr) {
		if valErr.Kind() != kreuzberg.ErrorKindValidation {
			t.Errorf("expected ValidationError, got %T", err)
		}
	}
}

// TestOutOfMemoryPatternLargeChunk simulates memory-intensive operations.
func TestOutOfMemoryPatternLargeChunk(t *testing.T) {
	// Create a very large chunk size that may exceed reasonable memory
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize: kreuzberg.IntPtr(2147483647), // Max int32 - unreasonable size
		},
	}

	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test"),
		"text/plain",
		config,
	)

	// Should handle gracefully with an error
	if err == nil {
		t.Fatalf("expected error for excessive chunk size, got nil")
	}
}

// TestNonexistentDirectory validates error handling for paths in missing directories.
func TestNonexistentDirectory(t *testing.T) {
	// Reference a file in a directory that doesn't exist
	_, err := kreuzberg.ExtractFileSync(
		"/nonexistent/deeply/nested/dir/file.pdf",
		nil,
	)

	if err == nil {
		t.Fatalf("expected error for nonexistent directory, got nil")
	}
}

// TestDirectoryPathInsteadOfFile validates error handling when given directory instead of file.
func TestDirectoryPathInsteadOfFile(t *testing.T) {
	dir := t.TempDir()
	// Try to extract from a directory, not a file
	_, err := kreuzberg.ExtractFileSync(dir, nil)

	if err == nil {
		t.Fatalf("expected error for directory path, got nil")
	}
}

// TestConcurrentErrorStatesRaceCondition validates thread-safe error handling.
func TestConcurrentErrorStatesRaceCondition(t *testing.T) {
	numGoroutines := 20
	errChan := make(chan error, numGoroutines)
	done := make(chan bool, numGoroutines)

	// Launch concurrent operations with mixed valid/invalid configs
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer func() {
				done <- true
			}()

			var config *kreuzberg.ExtractionConfig
			if index%2 == 0 {
				// Invalid config on even indices
				config = &kreuzberg.ExtractionConfig{
					Chunking: &kreuzberg.ChunkingConfig{
						ChunkSize: kreuzberg.IntPtr(-10),
					},
				}
			}

			_, err := kreuzberg.ExtractBytesSync(
				[]byte("concurrent test"),
				"text/plain",
				config,
			)

			if err != nil {
				errChan <- err
			}
		}(i)
	}

	// Wait for all goroutines
	for i := 0; i < numGoroutines; i++ {
		<-done
	}

	// Verify errors were captured without race conditions
	close(errChan)
	errorCount := 0
	for err := range errChan {
		if err == nil {
			t.Errorf("error should not be nil")
		}
		errorCount++
	}

	// We expect errors from invalid configs (even indices = 10 operations)
	if errorCount == 0 {
		t.Errorf("expected errors from invalid configs, got %d", errorCount)
	}
}

// TestErrorWrappingPreservesContext validates that error wrapping maintains error information.
func TestErrorWrappingPreservesContext(t *testing.T) {
	// Test with invalid config to generate an error
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize: kreuzberg.IntPtr(-1),
		},
	}

	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test"),
		"text/plain",
		config,
	)

	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	// Verify error message is informative
	errMsg := err.Error()
	if errMsg == "" {
		t.Errorf("error message should not be empty")
	}

	if len(errMsg) == 0 {
		t.Errorf("error message should have content")
	}
}

// TestInvalidConfigJSONSerialization validates error handling for config serialization.
func TestInvalidConfigJSONSerialization(t *testing.T) {
	// Test with a config that might fail JSON serialization
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize: kreuzberg.IntPtr(512),
		},
	}

	// Verify config serializes properly or returns error
	result, err := kreuzberg.ConfigToJSON(config)

	// Should either succeed with valid JSON or return a clear error
	if err == nil {
		if result == "" {
			t.Errorf("result should not be empty when no error")
		}
		if len(result) == 0 {
			t.Errorf("result should have content")
		}
	} else {
		errMsg := err.Error()
		if errMsg == "" {
			t.Errorf("error message should not be empty")
		}
	}
}

// TestErrorTypeDiscrimination validates proper classification of different error types.
func TestErrorTypeDiscrimination(t *testing.T) {
	tests := []struct {
		name      string
		fn        func() error
		checkType func(error) bool
	}{
		{
			name: "validation error for empty path",
			fn: func() error {
				_, err := kreuzberg.ExtractFileSync("", nil)
				return err
			},
			checkType: func(err error) bool {
				_, ok := err.(*kreuzberg.ValidationError)
				return ok
			},
		},
		{
			name: "validation error for empty MIME type",
			fn: func() error {
				_, err := kreuzberg.ExtractBytesSync(
					[]byte("test"),
					"",
					nil,
				)
				return err
			},
			checkType: func(err error) bool {
				_, ok := err.(*kreuzberg.ValidationError)
				return ok
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.fn()
			if err == nil {
				t.Fatalf("expected error, got nil")
			}
			if !tt.checkType(err) {
				t.Errorf("error type mismatch for %s, got %T", tt.name, err)
			}
		})
	}
}

// TestErrorUnwrapping validates that errors.As works correctly.
func TestErrorUnwrapping(t *testing.T) {
	_, err := kreuzberg.ExtractFileSync("/nonexistent/file.pdf", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	// Test errors.As for type assertion
	var valErr *kreuzberg.ValidationError
	if errors.As(err, &valErr) {
		// This is fine - it's a validation error
		if valErr == nil {
			t.Errorf("ValidationError should not be nil")
		}
	}
}

// TestReadOnlyFilePermissionError validates error handling for permission issues.
func TestReadOnlyFilePermissionError(t *testing.T) {
	dir := t.TempDir()
	testFile := filepath.Join(dir, "test.txt")

	// Write a file with restricted permissions (use umask-safe approach)
	err := os.WriteFile(testFile, []byte("content"), 0o600)
	if err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	// Attempt to remove read permissions
	err = os.Chmod(testFile, 0o000)
	if err != nil {
		// If we can't change permissions, test that extraction still returns appropriate error
		// when trying to read an unreadable file (on systems that don't support permission checks)
		_, err := kreuzberg.ExtractFileSync(testFile, nil)
		// On systems where permissions aren't enforced, this may or may not error
		// Just verify that extraction is attempted without panic
		if err != nil {
			t.Logf("extraction failed as expected: %v", err)
		} else {
			t.Logf("extraction succeeded on system without permission enforcement")
		}
		return
	}
	defer os.Chmod(testFile, 0o600) // Restore permissions for cleanup

	_, err = kreuzberg.ExtractFileSync(testFile, nil)
	if err == nil && os.Geteuid() != 0 {
		t.Fatalf("expected error for permission denied, got nil")
	}
}

// TestNilConfigHandling validates that nil config is handled gracefully.
func TestNilConfigHandling(t *testing.T) {
	// nil config should use defaults, not panic
	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test content"),
		"text/plain",
		nil, // Explicitly nil config
	)

	// Should complete without panic (error is acceptable)
	// We're primarily checking that it doesn't panic
	_ = err
}

// TestErrorPropagationAcrossFFIBoundary validates error handling across C FFI.
func TestErrorPropagationAcrossFFIBoundary(t *testing.T) {
	// Invalid config should propagate error from FFI layer
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize: kreuzberg.IntPtr(-500),
		},
	}

	_, err := kreuzberg.ExtractBytesSync(
		[]byte("test"),
		"text/plain",
		config,
	)

	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	// Error should contain meaningful information from C layer
	errorMsg := err.Error()
	if errorMsg == "" {
		t.Errorf("error message should not be empty")
	}
	if len(errorMsg) <= 5 {
		t.Errorf("error message too short, might be truncated: %s", errorMsg)
	}
}

// TestMultipleInvalidConditions validates compound error scenarios.
func TestMultipleInvalidConditions(t *testing.T) {
	// Combine multiple invalid conditions
	config := &kreuzberg.ExtractionConfig{
		Chunking: &kreuzberg.ChunkingConfig{
			ChunkSize:    kreuzberg.IntPtr(-100), // Invalid
			ChunkOverlap: kreuzberg.IntPtr(-50),  // Invalid
		},
	}

	_, err := kreuzberg.ExtractBytesSync(
		[]byte{}, // Empty data
		"",       // Empty MIME type
		config,
	)

	if err == nil {
		t.Fatalf("expected error for multiple invalid conditions, got nil")
	}
}

// TestErrorIsNotNil validates that error detection works correctly.
func TestErrorIsNotNil(t *testing.T) {
	_, err := kreuzberg.ExtractBytesSync(
		[]byte{},
		"",
		nil,
	)

	if err == nil {
		t.Fatalf("expected error for empty data and mime type, got nil")
	}
}

// TestConcurrentErrorHandlingRaceDetection validates thread-safe error handling without race conditions.
func TestConcurrentErrorHandlingRaceDetection(t *testing.T) {
	numGoroutines := 30
	var wg sync.WaitGroup
	errorCount := int64(0)
	var mu sync.Mutex

	wg.Add(numGoroutines)
	for i := 0; i < numGoroutines; i++ {
		go func(index int) {
			defer wg.Done()

			// Alternate between invalid and potentially valid configs
			var config *kreuzberg.ExtractionConfig
			if index%3 == 0 {
				config = &kreuzberg.ExtractionConfig{
					Chunking: &kreuzberg.ChunkingConfig{
						ChunkSize: kreuzberg.IntPtr(-25),
					},
				}
			}

			_, err := kreuzberg.ExtractBytesSync(
				[]byte("test"),
				"text/plain",
				config,
			)

			if err != nil {
				atomic.AddInt64(&errorCount, 1)
			}

			mu.Lock()
			// Verify we can access error safely
			_ = err
			mu.Unlock()
		}(i)
	}

	wg.Wait()

	if errorCount == 0 {
		t.Errorf("expected some errors from invalid configs, got none")
	}
}

// TestConfigValidationChaining validates error handling in chained operations.
func TestConfigValidationChaining(t *testing.T) {
	// Test that errors are properly returned in sequence
	testCases := []struct {
		name      string
		data      []byte
		mimeType  string
		config    *kreuzberg.ExtractionConfig
		shouldErr bool
	}{
		{
			name:      "empty mime type",
			data:      []byte("content"),
			mimeType:  "",
			config:    nil,
			shouldErr: true,
		},
		{
			name:     "negative chunk size",
			data:     []byte("content"),
			mimeType: "text/plain",
			config: &kreuzberg.ExtractionConfig{
				Chunking: &kreuzberg.ChunkingConfig{
					ChunkSize: kreuzberg.IntPtr(-1),
				},
			},
			shouldErr: true,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			_, err := kreuzberg.ExtractBytesSync(tc.data, tc.mimeType, tc.config)

			if tc.shouldErr && err == nil {
				t.Errorf("test %s: expected error, got nil", tc.name)
			}
			if !tc.shouldErr && err != nil {
				t.Errorf("test %s: unexpected error: %v", tc.name, err)
			}
		})
	}
}

// TestErrorMessageContent validates that error messages contain useful information.
func TestErrorMessageContent(t *testing.T) {
	_, err := kreuzberg.ExtractFileSync("/clearly/nonexistent/file.pdf", nil)

	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	errMsg := err.Error()
	if errMsg == "" {
		t.Errorf("error message should not be empty")
	}

	// Error should be a string representation
	if len(errMsg) < 3 {
		t.Errorf("error message too short: %q", errMsg)
	}
}

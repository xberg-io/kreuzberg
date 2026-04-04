package kreuzberg_test

import (
	"sync"
	"testing"

	kreuzberg "github.com/kreuzberg-dev/kreuzberg/packages/go/v4"
)

// TestMutexBasicProtection verifies that the FFI mutex prevents concurrent crashes
func TestMutexBasicProtection(t *testing.T) {
	var wg sync.WaitGroup
	errorCount := 0

	// Run 5 goroutines trying to get the library version concurrently
	// This tests basic FFI threading without PDF processing
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			version := kreuzberg.LibraryVersion()
			if version == "" {
				errorCount++
			}
		}()
	}

	wg.Wait()

	if errorCount > 0 {
		t.Errorf("expected 0 errors, got %d", errorCount)
	}
}

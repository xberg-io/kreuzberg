package e2e_test

import (
	"os"
	"path/filepath"
	"runtime"
	"testing"
)

func TestMain(m *testing.M) {
	_, filename, _, _ := runtime.Caller(0)
	dir := filepath.Dir(filename)

	// Change to the configured test-documents directory (if it exists) so that fixture
	// file paths like "pdf/fake_memo.pdf" resolve correctly when running go test
	// from e2e/go/. Repos without document fixtures skip chdir and run from e2e/go/.
	testDocumentsDir := filepath.Join(dir, "..", "..", "test_documents")
	if info, err := os.Stat(testDocumentsDir); err == nil && info.IsDir() {
		if err := os.Chdir(testDocumentsDir); err != nil {
			panic(err)
		}
	}

	os.Exit(m.Run())
}

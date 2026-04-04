package kreuzberg

import (
	"os"
	"testing"
)

// skipIfONNXNotAvailable skips the test if ONNX Runtime is not available (typically in CI without prebuilt binaries)
func skipIfONNXNotAvailable(t *testing.T) {
	// Skip if in CI or if SKIP_ONNX_TESTS is set
	if os.Getenv("IS_CI") == "true" || os.Getenv("SKIP_ONNX_TESTS") == "true" {
		t.Skip("Skipping due to missing ONNX Runtime - requires prebuilt binaries")
	}
}

// TestListEmbeddingPresets tests listing available embedding presets.
func TestListEmbeddingPresets(t *testing.T) {
	skipIfONNXNotAvailable(t)
	presets, err := ListEmbeddingPresets()
	if err != nil {
		t.Fatalf("list embedding presets: %v", err)
	}
	if len(presets) == 0 {
		t.Fatalf("expected at least one preset")
	}
}

// TestGetEmbeddingPreset tests retrieving specific embedding preset metadata.
func TestGetEmbeddingPreset(t *testing.T) {
	skipIfONNXNotAvailable(t)
	preset, err := GetEmbeddingPreset("balanced")
	if err != nil {
		t.Fatalf("get embedding preset: %v", err)
	}
	if preset == nil {
		t.Fatalf("preset should not be nil")
	}
	if preset.Name == "" || preset.ModelName == "" {
		t.Fatalf("preset fields missing: %+v", preset)
	}

	if _, err := GetEmbeddingPreset("nonexistent"); err == nil {
		t.Fatalf("expected error for unknown preset")
	}
}

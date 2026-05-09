```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/kreuzberg/packages/go/v5"
)

type CloudOcrBackend struct {
	apiKey         string
	supportedLangs []string
}

func (b *CloudOcrBackend) Name() string      { return "cloud-ocr" }
func (b *CloudOcrBackend) Version() string   { return "1.0.0" }
func (b *CloudOcrBackend) Initialize() error { return nil }
func (b *CloudOcrBackend) Shutdown() error   { return nil }

func (b *CloudOcrBackend) ProcessImage(
	imageBytes []byte,
	config map[string]interface{},
) (map[string]interface{}, error) {
	// Call your cloud OCR API with imageBytes and config["language"].
	return map[string]interface{}{
		"content":   "Extracted text",
		"mime_type": "text/plain",
	}, nil
}

func (b *CloudOcrBackend) ProcessImageFile(
	path interface{},
	config map[string]interface{},
) (map[string]interface{}, error) {
	return nil, fmt.Errorf("file-based OCR not implemented")
}

func (b *CloudOcrBackend) SupportsLanguage(lang string) bool {
	for _, l := range b.supportedLangs {
		if l == lang {
			return true
		}
	}
	return false
}

func (b *CloudOcrBackend) BackendType() map[string]interface{} {
	return map[string]interface{}{"Custom": "cloud-ocr"}
}

func (b *CloudOcrBackend) SupportedLanguages() []string {
	return b.supportedLangs
}

func (b *CloudOcrBackend) SupportsTableDetection() bool     { return false }
func (b *CloudOcrBackend) SupportsDocumentProcessing() bool { return false }

func (b *CloudOcrBackend) ProcessDocument(
	_ interface{},
	_ map[string]interface{},
) (map[string]interface{}, error) {
	return nil, fmt.Errorf("document processing not supported")
}

func main() {
	backend := &CloudOcrBackend{
		apiKey:         "your-api-key",
		supportedLangs: []string{"eng", "deu", "fra"},
	}

	if err := kreuzberg.RegisterOcrBackend(backend); err != nil {
		log.Fatalf("register failed: %v", err)
	}
	defer kreuzberg.UnregisterOcrBackend(backend.Name())

	lang := "eng"
	result, err := kreuzberg.ExtractFileSync("scanned.pdf", &kreuzberg.ExtractionConfig{
		OCR: &kreuzberg.OCRConfig{
			Backend:  "cloud-ocr",
			Language: &lang,
		},
	})
	if err != nil {
		log.Fatalf("extract failed: %v", err)
	}

	log.Println("content length:", len(result.Content))
}
```

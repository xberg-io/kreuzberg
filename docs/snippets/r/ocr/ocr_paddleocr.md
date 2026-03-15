```r title="R"
library(kreuzberg)

# Configure PaddleOCR backend with mobile model tier
ocr <- ocr_config(backend = "paddle-ocr", language = "en", model_tier = "mobile")
config <- extraction_config(force_ocr = TRUE, ocr = ocr)

# Extract text from an image using PaddleOCR
result <- extract_file_sync("document.jpg", config = config)

cat(sprintf("Extracted %d characters\n", nchar(result$content)))
cat(sprintf("MIME type: %s\n", result$mime_type))
cat("Content preview:\n")
cat(substr(result$content, 1, 200))
```

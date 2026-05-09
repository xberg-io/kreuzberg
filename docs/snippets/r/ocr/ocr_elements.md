```r title="R"
library(kreuzberg)

# Enable structured OCR elements alongside text extraction
element_cfg <- ocr_element_config(include_elements = TRUE)
ocr_cfg <- ocr_config(
  backend = "paddleocr",
  language = "en",
  element_config = element_cfg
)
config <- extraction_config(ocr = ocr_cfg)

result <- extract_file_sync("scanned.pdf", "application/pdf", config)

if (!is.null(result$ocr_elements)) {
  for (element in result$ocr_elements) {
    cat(sprintf("Text: %s\n", element$text))
    cat(sprintf("Confidence: %.2f\n", element$confidence$recognition))
    cat(sprintf("Geometry: %s\n", toString(element$geometry)))
    if (!is.null(element$rotation)) {
      cat(sprintf("Rotation: %s°\n", element$rotation$angle_degrees))
    }
    cat("\n")
  }
}
```

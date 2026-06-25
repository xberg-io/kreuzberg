```r title="R"
library(xberg)

# Clear all custom OCR backends
clear_ocr_backends()
cat("OCR backends cleared\n")

# Clear all custom validators
clear_validators()
cat("Validators cleared\n")

# Clear all custom post-processors
clear_post_processors()
cat("Post-processors cleared\n")

# Clear all custom document extractors
clear_document_extractors()
cat("Document extractors cleared\n")
```

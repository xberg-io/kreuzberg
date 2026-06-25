```r
library(xberg)

# List available OCR backends
backends <- list_ocr_backends()
cat("Available backends:", paste(backends, collapse = ", "), "\n")

# List registered post-processors
processors <- list_post_processors()
cat("Post-processors:", paste(processors, collapse = ", "), "\n")

# Clear all custom registrations
clear_post_processors()
clear_validators()
```

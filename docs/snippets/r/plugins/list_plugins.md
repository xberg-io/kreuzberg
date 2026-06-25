```r title="R"
library(xberg)

ocr_backends <- list_ocr_backends()
cat(sprintf("OCR backends: %s\n", paste(ocr_backends, collapse=", ")))

validators <- list_validators()
cat(sprintf("Validators: %s\n", paste(validators, collapse=", ")))

post_processors <- list_post_processors()
cat(sprintf("Post-processors: %s\n", paste(post_processors, collapse=", ")))

extractors <- list_document_extractors()
cat(sprintf("Document extractors: %s\n", paste(extractors, collapse=", ")))
```

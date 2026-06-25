```r title="R"
library(xberg)

# Confirm the native extension loaded by listing registered extractors
extractors <- list_document_extractors()
cat(sprintf("xberg ready: %d document extractors registered\n", length(extractors)))
```

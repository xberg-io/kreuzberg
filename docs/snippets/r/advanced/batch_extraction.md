```r
library(xberg)

# Batch extract from multiple files
files <- c("report.pdf", "slides.pptx", "data.xlsx")
results <- batch_extract_files_sync(files)

for (i in seq_along(results)) {
  cat(sprintf("File: %s\n", files[i]))
  cat(sprintf("  MIME: %s\n", results[[i]]$mime_type))
  cat(sprintf("  Length: %d chars\n\n", nchar(results[[i]]$content)))
}
```

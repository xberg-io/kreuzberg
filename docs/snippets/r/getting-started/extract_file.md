```r title="R"
library(xberg)

# Extract a file and inspect the result
result <- extract_file_sync("document.pdf")

# Print result information
cat(sprintf("MIME type: %s\n", mime_type(result)))
cat(sprintf("Content length: %d characters\n", nchar(content(result))))
cat(sprintf("Page count: %d\n", page_count(result)))

# View additional metadata
cat(sprintf("Detected language: %s\n", detected_language(result)))
```

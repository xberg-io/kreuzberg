```r
library(xberg)

# Extract text from a PDF file
result <- extract_file_sync("document.pdf")
cat(result$content)
```

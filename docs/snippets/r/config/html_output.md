```r title="R"
library(kreuzberg)

html_cfg <- html_output_config(
  theme = "git_hub",
  embed_css = TRUE
)

config <- extraction_config(
  output_format = "html",
  html_output = html_cfg
)

result <- extract_file_sync("document.pdf", "application/pdf", config)
cat(result$content) # HTML with kb-* classes
```

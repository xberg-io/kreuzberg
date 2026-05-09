```r title="R"
library(kreuzberg)

hierarchy_cfg <- hierarchy_config(
  enabled = TRUE,
  k_clusters = 6L,
  include_bbox = TRUE,
  ocr_coverage_threshold = 0.8
)

pdf_cfg <- pdf_config(
  extract_metadata = TRUE,
  hierarchy = hierarchy_cfg
)

config <- extraction_config(pdf_options = pdf_cfg)

result <- extract_file_sync("document.pdf", "application/pdf", config)
cat(sprintf("Pages: %d\n", length(result$pages)))
```

```r title="R"
library(kreuzberg)

embedding_cfg <- embedding_config(
  model = list(type = "preset", name = "balanced"),
  batch_size = 16L,
  normalize = TRUE,
  show_download_progress = TRUE
)

chunking_cfg <- chunking_config(
  max_characters = 1000L,
  overlap = 200L,
  embedding = embedding_cfg
)

config <- extraction_config(chunking = chunking_cfg)

result <- extract_file_sync("document.pdf", "application/pdf", config)
cat(sprintf("Chunks with embeddings: %d\n", length(result$chunks)))
```

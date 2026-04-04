# Extracted from test-contract.R:65

# test -------------------------------------------------------------------------
result <- run_fixture_with_method(
  "api_batch_file_sync",
  "pdf/fake_memo.pdf",
  NULL,
  "batch_sync",
  "file",
  requirements = character(0),
  notes = NULL,
  skip_if_missing = TRUE
)

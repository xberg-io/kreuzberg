# Extracted from test-contract.R:33

# test -------------------------------------------------------------------------
result <- run_fixture_with_method(
  "api_batch_bytes_sync",
  "pdf/fake_memo.pdf",
  NULL,
  "batch_sync",
  "bytes",
  requirements = character(0),
  notes = NULL,
  skip_if_missing = TRUE
)

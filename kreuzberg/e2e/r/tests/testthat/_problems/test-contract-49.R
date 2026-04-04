# Extracted from test-contract.R:49

# test -------------------------------------------------------------------------
result <- run_fixture_with_method(
  "api_batch_file_async",
  "pdf/fake_memo.pdf",
  NULL,
  "batch_async",
  "file",
  requirements = character(0),
  notes = NULL,
  skip_if_missing = TRUE
)

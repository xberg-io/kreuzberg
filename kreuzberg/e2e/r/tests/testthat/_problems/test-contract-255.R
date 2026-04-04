# Extracted from test-contract.R:255

# test -------------------------------------------------------------------------
skip_if_feature_unavailable("office")
result <- run_fixture(
  "config_element_types",
  "docx/unit_test_headers.docx",
  list(result_format = "element_based"),
  requirements = c("office"),
  notes = NULL,
  skip_if_missing = TRUE
)
assert_expected_mime(result, c("application/vnd.openxmlformats-officedocument.wordprocessingml.document"))
assert_elements(result, min_count = 1L, types_include = c("title", "narrative_text"))

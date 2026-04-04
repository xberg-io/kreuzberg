test_that("list_post_processors returns a character vector", {
  result <- list_post_processors()
  expect_true(is.character(result) || is.list(result))
})

test_that("list_validators returns a character vector", {
  result <- list_validators()
  expect_true(is.character(result) || is.list(result))
})

test_that("list_ocr_backends returns a character vector", {
  result <- list_ocr_backends()
  expect_true(is.character(result) || is.list(result))
})

test_that("clear_post_processors does not error", {
  expect_no_error(clear_post_processors())
})

test_that("clear_validators does not error", {
  expect_no_error(clear_validators())
})

test_that("clear_ocr_backends does not error", {
  expect_no_error(clear_ocr_backends())
})

test_that("unregister_post_processor handles missing gracefully", {
  expect_no_error(unregister_post_processor("nonexistent-xyz"))
})

test_that("unregister_validator handles missing gracefully", {
  expect_no_error(unregister_validator("nonexistent-xyz"))
})

test_that("unregister_ocr_backend handles missing gracefully", {
  expect_no_error(unregister_ocr_backend("nonexistent-xyz"))
})

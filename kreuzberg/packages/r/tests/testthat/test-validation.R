test_that("validate_ocr_backend_name accepts known backends", {
  expect_true(validate_ocr_backend_name("tesseract"))
})

test_that("validate_ocr_backend_name rejects invalid backends", {
  expect_error(validate_ocr_backend_name("totally_fake_backend"))
})

test_that("validate_ocr_backend_name validates input type", {
  expect_error(validate_ocr_backend_name(123))
  expect_error(validate_ocr_backend_name(c("a", "b")))
})

test_that("validate_language_code accepts valid codes", {
  expect_true(validate_language_code("eng"))
  expect_true(validate_language_code("deu"))
})

test_that("validate_language_code validates input type", {
  expect_error(validate_language_code(123))
})

test_that("validate_output_format accepts known formats", {
  expect_true(validate_output_format("text"))
  expect_true(validate_output_format("markdown"))
  expect_true(validate_output_format("html"))
})

test_that("validate_output_format rejects invalid formats", {
  expect_error(validate_output_format("not_a_format"))
})

test_that("validate_output_format validates input type", {
  expect_error(validate_output_format(123))
})

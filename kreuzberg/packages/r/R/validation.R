#' Validate an OCR backend name
#'
#' Checks whether the given string is a recognized OCR backend name.
#'
#' @param backend Character string naming the OCR backend (e.g., "tesseract", "paddle-ocr").
#' @return Logical indicating if the backend name is valid.
#' @export
validate_ocr_backend_name <- function(backend) {
  stopifnot(is.character(backend), length(backend) == 1L)
  check_native_result(validate_ocr_backend_name_native(backend))
}

#' Validate a language code
#'
#' Checks whether the given string is a valid language code for OCR
#' (ISO 639-1 two-letter or ISO 639-3 three-letter codes).
#'
#' @param code Character string language code (e.g., "eng", "deu", "en").
#' @return Logical indicating if the code is valid.
#' @export
validate_language_code <- function(code) {
  stopifnot(is.character(code), length(code) == 1L)
  check_native_result(validate_language_code_native(code))
}

#' Validate an output format
#'
#' Checks whether the given string is a supported output format.
#'
#' @param format Character string output format (e.g., "text", "markdown", "html", "json").
#' @return Logical indicating if the format is valid.
#' @export
validate_output_format <- function(format) {
  stopifnot(is.character(format), length(format) == 1L)
  check_native_result(validate_output_format_native(format))
}

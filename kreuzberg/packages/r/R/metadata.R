#' Detect MIME type from raw bytes
#'
#' Analyzes the content of a raw byte vector to determine the MIME type.
#'
#' @param data Raw vector of bytes.
#' @return Character string with detected MIME type.
#' @export
detect_mime_type <- function(data) {
  stopifnot(is.raw(data))
  check_native_result(detect_mime_type_native(data))
}

#' Detect MIME type from file path
#'
#' Determines the MIME type of a file using both its extension and content.
#'
#' @param path Character string path to the file.
#' @return Character string with detected MIME type.
#' @export
detect_mime_type_from_path <- function(path) {
  stopifnot(is.character(path), length(path) == 1L)
  check_native_result(detect_mime_type_from_path_native(path))
}

#' Get file extensions for a MIME type
#'
#' Returns the known file extensions associated with a given MIME type.
#'
#' @param mime_type Character string MIME type (e.g., "application/pdf").
#' @return Character vector of file extensions (e.g., c("pdf")).
#' @export
get_extensions_for_mime <- function(mime_type) {
  stopifnot(is.character(mime_type), length(mime_type) == 1L)
  check_native_result(get_extensions_for_mime_native(mime_type))
}

#' Validate a MIME type string
#'
#' Checks whether a given string is a valid and supported MIME type.
#'
#' @param mime_type Character string MIME type to validate.
#' @return Logical indicating if the MIME type is valid.
#' @export
validate_mime_type <- function(mime_type) {
  stopifnot(is.character(mime_type), length(mime_type) == 1L)
  check_native_result(validate_mime_type_native(mime_type))
}

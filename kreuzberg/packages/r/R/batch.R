#' Batch extract content from multiple files (synchronous)
#'
#' @param paths Character vector of file paths.
#' @param config Optional extraction configuration.
#' @return A list of \code{kreuzberg_result} objects.
#' @export
batch_extract_files_sync <- function(paths, config = NULL) {
  paths <- as.character(paths)
  stopifnot(length(paths) > 0L)
  config_json <- if (!is.null(config)) jsonlite::toJSON(config, auto_unbox = TRUE) else NULL
  results <- check_native_result(batch_extract_files_sync_native(paths, config_json))
  lapply(results, as_kreuzberg_result)
}

#' Batch extract content from multiple files (async, blocks in R)
#'
#' @param paths Character vector of file paths.
#' @param config Optional extraction configuration.
#' @return A list of \code{kreuzberg_result} objects.
#' @export
batch_extract_files <- function(paths, config = NULL) {
  paths <- as.character(paths)
  stopifnot(length(paths) > 0L)
  config_json <- if (!is.null(config)) jsonlite::toJSON(config, auto_unbox = TRUE) else NULL
  results <- check_native_result(batch_extract_files_native(paths, config_json))
  lapply(results, as_kreuzberg_result)
}

#' Batch extract content from multiple byte arrays (synchronous)
#'
#' @param data_list List of raw vectors.
#' @param mime_types Character vector of MIME types (one per item).
#' @param config Optional extraction configuration.
#' @return A list of \code{kreuzberg_result} objects.
#' @export
batch_extract_bytes_sync <- function(data_list, mime_types, config = NULL) {
  stopifnot(is.list(data_list), length(data_list) > 0L)
  mime_types <- as.character(mime_types)
  if (length(data_list) != length(mime_types)) {
    stop("data_list and mime_types must have the same length", call. = FALSE)
  }
  config_json <- if (!is.null(config)) jsonlite::toJSON(config, auto_unbox = TRUE) else NULL
  results <- check_native_result(batch_extract_bytes_sync_native(data_list, mime_types, config_json))
  lapply(results, as_kreuzberg_result)
}

#' Batch extract content from multiple byte arrays (async, blocks in R)
#'
#' @param data_list List of raw vectors.
#' @param mime_types Character vector of MIME types (one per item).
#' @param config Optional extraction configuration.
#' @return A list of \code{kreuzberg_result} objects.
#' @export
batch_extract_bytes <- function(data_list, mime_types, config = NULL) {
  stopifnot(is.list(data_list), length(data_list) > 0L)
  mime_types <- as.character(mime_types)
  if (length(data_list) != length(mime_types)) {
    stop("data_list and mime_types must have the same length", call. = FALSE)
  }
  config_json <- if (!is.null(config)) jsonlite::toJSON(config, auto_unbox = TRUE) else NULL
  results <- check_native_result(batch_extract_bytes_native(data_list, mime_types, config_json))
  lapply(results, as_kreuzberg_result)
}

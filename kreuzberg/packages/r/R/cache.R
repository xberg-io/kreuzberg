#' Clear the extraction cache
#'
#' Removes all cached extraction results from the kreuzberg cache directory.
#'
#' @return Invisible NULL on success.
#' @export
clear_cache <- function() {
  check_native_result(clear_cache_native())
}

#' Get cache statistics
#'
#' Returns information about the current state of the extraction cache.
#'
#' @return A named list with:
#'   \describe{
#'     \item{total_entries}{Integer count of cached items.}
#'     \item{total_size_bytes}{Numeric total size in bytes.}
#'   }
#' @export
cache_stats <- function() {
  check_native_result(cache_stats_native())
}

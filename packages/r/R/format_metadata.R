#' Format-specific metadata types
#'
#' When extraction results include format-specific metadata, the
#' \code{metadata$format_type} field indicates which variant is present.
#' The remaining fields in the flattened metadata map correspond to the
#' variant's fields described below.
#'
#' R handles JSON natively via lists, so these types exist primarily for
#' documentation and validation purposes.
#'
#' @section CSV metadata (\code{format_type = "csv"}):
#' \describe{
#'   \item{row_count}{Integer. Number of rows.}
#'   \item{column_count}{Integer. Number of columns.}
#'   \item{delimiter}{Character or NULL. Delimiter character.}
#'   \item{has_header}{Logical. Whether the file has a header row.}
#'   \item{column_types}{Character vector or NULL. Detected column types.}
#' }
#'
#' @section BibTeX metadata (\code{format_type = "bibtex"}):
#' \describe{
#'   \item{entry_count}{Integer. Number of BibTeX entries.}
#'   \item{citation_keys}{Character vector. Citation keys.}
#'   \item{authors}{Character vector. Author names.}
#'   \item{year_range}{Named list or NULL with \code{min}, \code{max}, \code{years}.}
#'   \item{entry_types}{Named list or NULL. Entry type to count mapping.}
#' }
#'
#' @section Citation metadata (\code{format_type = "citation"}):
#' \describe{
#'   \item{citation_count}{Integer. Number of citations.}
#'   \item{format}{Character or NULL. Citation format (e.g., "RIS", "PubMed").}
#'   \item{authors}{Character vector. Author names.}
#'   \item{year_range}{Named list or NULL with \code{min}, \code{max}, \code{years}.}
#'   \item{dois}{Character vector. DOIs.}
#'   \item{keywords}{Character vector. Keywords.}
#' }
#'
#' @section FictionBook metadata (\code{format_type = "fiction_book"}):
#' \describe{
#'   \item{genres}{Character vector. Genres.}
#'   \item{sequences}{Character vector. Series/sequences.}
#'   \item{annotation}{Character or NULL. Book annotation.}
#' }
#'
#' @section DBF metadata (\code{format_type = "dbf"}):
#' \describe{
#'   \item{record_count}{Integer. Number of records.}
#'   \item{field_count}{Integer. Number of fields.}
#'   \item{fields}{List of named lists, each with \code{name} and \code{field_type}.}
#' }
#'
#' @section JATS metadata (\code{format_type = "jats"}):
#' \describe{
#'   \item{copyright}{Character or NULL. Copyright statement.}
#'   \item{license}{Character or NULL. License information.}
#'   \item{history_dates}{Named list. Publication history dates.}
#'   \item{contributor_roles}{List of named lists, each with \code{name} and \code{role}.}
#' }
#'
#' @section EPUB metadata (\code{format_type = "epub"}):
#' \describe{
#'   \item{coverage}{Character or NULL. Geographic/temporal coverage.}
#'   \item{dc_format}{Character or NULL. Dublin Core format.}
#'   \item{relation}{Character or NULL. Related resource.}
#'   \item{source}{Character or NULL. Source resource.}
#'   \item{dc_type}{Character or NULL. Dublin Core type.}
#'   \item{cover_image}{Character or NULL. Cover image path.}
#' }
#'
#' @section PST metadata (\code{format_type = "pst"}):
#' \describe{
#'   \item{message_count}{Integer. Number of messages in the archive.}
#' }
#'
#' @name format_metadata
#' @family metadata
NULL

#' Check if metadata has a specific format type
#'
#' @param metadata A metadata list from an extraction result.
#' @param format_type Character string of the expected format type
#'   (e.g., "csv", "bibtex", "citation", "fiction_book", "dbf", "jats", "epub", "pst").
#' @return Logical indicating if the metadata matches the given format type.
#' @export
is_format_type <- function(metadata, format_type) {
  stopifnot(is.list(metadata), is.character(format_type), length(format_type) == 1L)
  identical(metadata[["format_type"]], format_type)
}

#' Extract format-specific metadata fields
#'
#' Returns only the format-specific fields from a metadata list,
#' excluding common metadata fields (title, authors, etc.).
#'
#' @param metadata A metadata list from an extraction result.
#' @return Named list of format-specific fields, or NULL if no format_type is present.
#' @export
format_metadata_fields <- function(metadata) {
  if (is.null(metadata) || is.null(metadata[["format_type"]])) {
    return(NULL)
  }

  common_keys <- c(
    "title", "subject", "authors", "keywords", "language",
    "created_at", "modified_at", "created_by", "modified_by",
    "pages", "image_preprocessing", "json_schema", "error",
    "category", "tags", "document_version", "abstract_text",
    "output_format", "extraction_duration_ms"
  )

  format_keys <- setdiff(names(metadata), common_keys)
  metadata[format_keys]
}

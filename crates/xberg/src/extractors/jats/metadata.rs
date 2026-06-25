//! JATS metadata extraction (authors, DOI, dates, journal information).

/// Structure to hold extracted JATS metadata.
#[derive(Debug, Clone, Default)]
pub(super) struct JatsMetadataExtracted {
    pub(super) title: String,
    pub(super) subtitle: Option<String>,
    pub(super) authors: Vec<String>,
    pub(super) affiliations: Vec<String>,
    pub(super) doi: Option<String>,
    pub(super) pii: Option<String>,
    pub(super) keywords: Vec<String>,
    pub(super) publication_date: Option<String>,
    pub(super) volume: Option<String>,
    pub(super) issue: Option<String>,
    pub(super) pages: Option<String>,
    pub(super) journal_title: Option<String>,
    pub(super) article_type: Option<String>,
    pub(super) abstract_text: Option<String>,
    pub(super) corresponding_author: Option<String>,
    /// History dates: (date-type, date-string) pairs, e.g. ("received", "2024-01-15")
    pub(super) history_dates: Vec<(String, String)>,
    /// Copyright statement from `<permissions>/<copyright-statement>`
    pub(super) copyright_statement: Option<String>,
    /// License text from `<permissions>/<license>`
    pub(super) license: Option<String>,
    /// Contributor roles: (name, role) pairs
    pub(super) contributor_roles: Vec<(String, String)>,
}

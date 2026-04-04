//! Render an `InternalDocument` to HTML5 via comrak.

use comrak::{Arena, format_html};

use crate::types::internal::InternalDocument;

use super::comrak_bridge::build_comrak_ast;
use super::markdown::comrak_options;

/// Render an `InternalDocument` to HTML5.
pub fn render_html(doc: &InternalDocument) -> String {
    let arena = Arena::new();
    let root = build_comrak_ast(doc, &arena);

    let mut options = comrak_options();
    options.render.r#unsafe = true; // allow raw HTML passthrough
    options.render.github_pre_lang = true; // <pre lang="X"><code> instead of <code class="language-X">
    options.render.full_info_string = true; // preserve full info string in data-meta attribute

    let mut output = String::new();
    format_html(root, &options, &mut output).expect("comrak HTML formatting should not fail");
    output
}

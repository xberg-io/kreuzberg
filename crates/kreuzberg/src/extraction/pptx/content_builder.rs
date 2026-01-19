//! Content builder for accumulating slide output.
//!
//! This module provides utilities for building the final markdown content
//! from slide elements and managing page boundaries.

pub(super) struct ContentBuilder {
    pub(super) content: String,
    pub(super) boundaries: Vec<crate::types::PageBoundary>,
    pub(super) page_contents: Vec<crate::types::PageContent>,
    pub(super) config: Option<crate::core::config::PageConfig>,
}

impl ContentBuilder {
    pub(super) fn new() -> Self {
        Self {
            content: String::with_capacity(8192),
            boundaries: Vec::new(),
            page_contents: Vec::new(),
            config: None,
        }
    }

    pub(super) fn with_page_config(capacity: usize, config: Option<crate::core::config::PageConfig>) -> Self {
        Self {
            content: String::with_capacity(capacity),
            boundaries: if config.is_some() {
                Vec::new()
            } else {
                Vec::with_capacity(0)
            },
            page_contents: if config.is_some() {
                Vec::new()
            } else {
                Vec::with_capacity(0)
            },
            config,
        }
    }

    pub(super) fn start_slide(&mut self, slide_number: u32) -> usize {
        let byte_start = self.content.len();

        if let Some(ref cfg) = self.config
            && cfg.insert_page_markers
        {
            let marker = cfg.marker_format.replace("{page_num}", &slide_number.to_string());
            self.content.push_str(&marker);
        }

        byte_start
    }

    pub(super) fn end_slide(&mut self, slide_number: u32, byte_start: usize, slide_content: String) {
        let byte_end = self.content.len();

        if self.config.is_some() {
            self.boundaries.push(crate::types::PageBoundary {
                byte_start,
                byte_end,
                page_number: slide_number as usize,
            });

            self.page_contents.push(crate::types::PageContent {
                page_number: slide_number as usize,
                content: slide_content,
                tables: Vec::new(),
                images: Vec::new(),
                hierarchy: None,
            });
        }
    }

    pub(super) fn add_slide_header(&mut self, slide_number: u32) {
        self.content.reserve(50);
        self.content.push_str("\n\n<!-- Slide number: ");
        self.content.push_str(&slide_number.to_string());
        self.content.push_str(" -->\n");
    }

    pub(super) fn add_text(&mut self, text: &str) {
        if !text.trim().is_empty() {
            self.content.push_str(text);
        }
    }

    pub(super) fn add_title(&mut self, title: &str) {
        if !title.trim().is_empty() {
            self.content.push_str("# ");
            self.content.push_str(title.trim());
            self.content.push('\n');
        }
    }

    pub(super) fn add_table(&mut self, rows: &[Vec<String>]) {
        if rows.is_empty() {
            return;
        }

        self.content.push_str("\n<table>");
        for (i, row) in rows.iter().enumerate() {
            self.content.push_str("<tr>");
            let tag = if i == 0 { "th" } else { "td" };

            for cell in row {
                self.content.push('<');
                self.content.push_str(tag);
                self.content.push('>');
                self.content.push_str(&super::image_handling::html_escape(cell));
                self.content.push_str("</");
                self.content.push_str(tag);
                self.content.push('>');
            }
            self.content.push_str("</tr>");
        }
        self.content.push_str("</table>\n");
    }

    pub(super) fn add_list_item(&mut self, level: u32, is_ordered: bool, text: &str) {
        let indent_count = level.saturating_sub(1) as usize;
        for _ in 0..indent_count {
            self.content.push_str("  ");
        }

        let marker = if is_ordered { "1." } else { "-" };
        self.content.push_str(marker);
        self.content.push(' ');
        self.content.push_str(text.trim());
        self.content.push('\n');
    }

    pub(super) fn add_image(&mut self, image_id: &str, slide_number: u32) {
        let filename = format!("slide_{}_image_{}.jpg", slide_number, image_id);
        self.content.push_str("![");
        self.content.push_str(image_id);
        self.content.push_str("](");
        self.content.push_str(&filename);
        self.content.push_str(")\n");
    }

    pub(super) fn add_notes(&mut self, notes: &str) {
        if !notes.trim().is_empty() {
            self.content.push_str("\n\n### Notes:\n");
            self.content.push_str(notes);
            self.content.push('\n');
        }
    }

    pub(super) fn build(
        self,
    ) -> (
        String,
        Option<Vec<crate::types::PageBoundary>>,
        Option<Vec<crate::types::PageContent>>,
    ) {
        let content = self.content.trim().to_string();
        let boundaries = if self.config.is_some() && !self.boundaries.is_empty() {
            Some(self.boundaries)
        } else {
            None
        };
        let pages = if self.config.is_some() && !self.page_contents.is_empty() {
            Some(self.page_contents)
        } else {
            None
        };
        (content, boundaries, pages)
    }
}

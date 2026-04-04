//! Fast benchmark groups: curated document subsets for targeted iteration.

/// A named benchmark group with a description and list of doc name patterns.
pub struct BenchmarkGroup {
    pub name: &'static str,
    pub description: &'static str,
    /// Document name patterns (matched via `contains`, same as --doc).
    pub docs: &'static [&'static str],
}

pub const GROUPS: &[BenchmarkGroup] = &[
    BenchmarkGroup {
        name: "tables",
        description: "Table extraction quality (wide tables, borderless, receipts)",
        docs: &[
            "senate-expenditures",
            "nics-background-checks-2015-11",
            "SPARSE-2024-INV-1234_borderless_table",
            "RECEIPT-2024-TXN-98765_retail_purchase",
            "REPAIR-2022-INV-001_multipage",
            "redp5110_sampled",
            "table-curves-example",
        ],
    },
    BenchmarkGroup {
        name: "structure",
        description: "Heading/structure detection (SF1 regressions)",
        docs: &[
            "pdfa_040",
            "nougat_028",
            "nougat_018",
            "pdfa_033",
            "pdf_structure",
            "hello_structure",
            "word365_structure",
            "figure_structure",
        ],
    },
    BenchmarkGroup {
        name: "multicolumn",
        description: "Multi-column and magazine-style layouts",
        docs: &[
            "nougat_028",
            "2305.03393v1",
            "2206.01062",
            "2203.01017v2",
            "federal-register-2020-17221",
        ],
    },
    BenchmarkGroup {
        name: "text-quality",
        description: "RTL, special chars, encoding, OCR edge cases",
        docs: &[
            "right_to_left_02",
            "right_to_left_03",
            "annotations-unicode-issues",
            "pdfa_033",
            "test-punkt",
            "issue-1114-dedupe-chars",
        ],
    },
    BenchmarkGroup {
        name: "ocr-fallback",
        description: "Documents where native extraction fails and OCR should trigger",
        docs: &[
            "senate-expenditures",
            "la-precinct-bulletin-2014-p1",
            "scotus-transcript-p1",
            "issue-848",
            "nics-background-checks-2015-11-rotated",
        ],
    },
];

/// Find a group by name, case-insensitive.
pub fn find_group(name: &str) -> Option<&'static BenchmarkGroup> {
    GROUPS.iter().find(|g| g.name.eq_ignore_ascii_case(name))
}

/// List all available group names.
pub fn group_names() -> Vec<&'static str> {
    GROUPS.iter().map(|g| g.name).collect()
}

Use Xberg::{extract, ExtractionConfig, ExtractInput, PageConfig};

Let config = ExtractionConfig {
pages: Some(PageConfig {
extract_pages: true,
..Default::default()
}),
..Default::default()
};

Let output = extract(ExtractInput::from_uri("document.pdf"), &config).await?;

If let Some(pages) = output.results[0].pages {
for page in pages {
println!("Page {}:", page.page_number);
println!(" Content: {} chars", page.content.len());
println!(" Tables: {}", page.tables.len());
println!(" Images: {}", page.images.len());
}
}

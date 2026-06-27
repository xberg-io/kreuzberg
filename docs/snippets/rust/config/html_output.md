```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, HtmlOutputConfig, HtmlTheme, OutputFormat};

let config = ExtractionConfig {
    output_format: OutputFormat::Html,
    html_output: Some(HtmlOutputConfig {
        theme: HtmlTheme::GitHub,
        ..Default::default()
    }),
    ..Default::default()
};
let output = extract(ExtractInput::from_uri("document.pdf"), &config).await.unwrap();
println!("{}", output.results[0].content); // HTML with kb-* classes
```

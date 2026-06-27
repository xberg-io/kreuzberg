```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, SummarizationConfig};
use xberg::types::summary::SummaryStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ExtractionConfig {
        summarization: Some(SummarizationConfig {
            strategy: SummaryStrategy::Extractive,
            max_tokens: Some(200),
            llm: None,
        }),
        ..Default::default()
    };
    let output = extract(ExtractInput::from_uri("report.pdf"), &config).await?;
    if let Some(summary) = &output.results[0].summary {
        println!("{}", summary.text);
    }
    Ok(())
}
```

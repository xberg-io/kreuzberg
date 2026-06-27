```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput, XbergError};

#[tokio::main]
async fn main() {
    let config = ExtractionConfig::default();
    match extract(ExtractInput::from_uri("document.pdf"), &config).await {
        Ok(output) => println!("{}", output.results[0].content),
        Err(XbergError::Io(e)) => eprintln!("File error: {e}"),
        Err(XbergError::UnsupportedFormat(mime)) => {
            eprintln!("Unsupported format: {mime}");
        }
        Err(XbergError::Parsing { message, .. }) => {
            eprintln!("Corrupt or invalid document: {message}");
        }
        Err(XbergError::MissingDependency(dep)) => {
            eprintln!("Missing dependency — install {dep}");
        }
        Err(e) => eprintln!("Extraction failed: {e}"),
    }
}
```

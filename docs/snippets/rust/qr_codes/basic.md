```rust title="Rust"
use xberg::{extract, ExtractionConfig, ExtractInput};

let config = ExtractionConfig {
    qr_codes: Some(true),
    ..Default::default()
};
let output = extract(ExtractInput::from_uri("ticket.pdf"), &config).await?;
for image in &output.results[0].images {
    if let Some(qrs) = &image.qr_codes {
        for qr in qrs {
            println!("{}", qr.payload);
        }
    }
}
```

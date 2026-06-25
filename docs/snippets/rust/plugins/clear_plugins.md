```rust title="Rust"
use xberg::{clear_document_extractors, clear_post_processors, clear_ocr_backends, clear_validators};

fn main() {
    clear_document_extractors();
    clear_post_processors();
    clear_ocr_backends();
    clear_validators();

    println!("All plugins cleared");
}
```

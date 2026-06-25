```zig title="Zig"
const std = @import("std");
const xberg = @import("xberg");

pub fn main() !void {
    var out_error: ?[*c]u8 = null;

    _ = xberg.unregister_post_processor("word-count", &out_error);
    _ = xberg.unregister_validator("min-length-validator", &out_error);
    _ = xberg.unregister_ocr_backend("custom-ocr", &out_error);
    _ = xberg.unregister_embedding_backend("my-embedder", &out_error);
}
```

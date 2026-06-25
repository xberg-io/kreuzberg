```zig title="Zig"
const std = @import("std");
const xberg = @import("xberg");

pub fn main() !void {
    try xberg.clear_ocr_backends();
    try xberg.clear_post_processors();
    try xberg.clear_validators();

    const stdout = std.io.getStdOut().writer();
    try stdout.print("All plugins cleared\n", .{});
}
```

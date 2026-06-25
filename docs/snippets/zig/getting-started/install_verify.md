```zig title="Zig"
const std = @import("std");
const xberg = @import("xberg");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    _ = allocator;

    const stdout = std.io.getStdOut().writer();
    try stdout.print("xberg module imported successfully\n", .{});
    if (xberg._last_error()) |context| {
        try stdout.print("  last error context: {s}\n", .{context});
    } else {
        try stdout.print("  no prior FFI errors recorded\n", .{});
    }
}
```

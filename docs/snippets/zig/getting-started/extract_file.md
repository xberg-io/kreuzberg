```zig title="Zig"
const std = @import("std");
const xberg = @import("xberg");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const config_json = "{}";
    const result_json = try xberg.extract_file_sync("document.pdf", null, config_json);
    defer std.heap.c_allocator.free(result_json);

    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, result_json, .{});
    defer parsed.deinit();

    const root = parsed.value.object;
    const content = root.get("content") orelse std.json.Value{ .string = "" };
    const mime_type = root.get("mime_type") orelse std.json.Value{ .string = "" };

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Content: {s}\n", .{content.string});
    try stdout.print("MIME Type: {s}\n", .{mime_type.string});
}
```

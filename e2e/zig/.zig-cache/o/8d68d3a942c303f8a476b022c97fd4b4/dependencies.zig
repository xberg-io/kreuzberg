pub const packages = struct {
    pub const @"../../packages/zig" = struct {
        pub const build_root = "/Users/naamanhirschfeld/workspace/kreuzberg-dev/kreuzberg/e2e/zig/../../packages/zig";
        pub const build_zig = @import("../../packages/zig");
        pub const deps: []const struct { []const u8, []const u8 } = &.{
        };
    };
};

pub const root_deps: []const struct { []const u8, []const u8 } = &.{
    .{ "kreuzberg", "../../packages/zig" },
};

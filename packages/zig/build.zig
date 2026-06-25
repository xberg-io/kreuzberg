const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Default library/include search paths follow the conventional Cargo workspace
    // layout. `alef publish package --lang zig` rewrites this file for the
    // distributed tarball so consumers link the bundled lib/ and include/ dirs.
    // Override with -Dffi_path=... and -Dffi_include_path=... if your layout differs.
    const ffi_path = b.option(
        []const u8,
        "ffi_path",
        "Path to directory containing libxberg_ffi.{dylib,so,dll,a}"
    ) orelse "../../target/release";

    const ffi_include = b.option(
        []const u8,
        "ffi_include_path",
        "Path to directory containing the FFI C header"
    ) orelse "../../crates/xberg-ffi/include";

    const module = b.addModule("xberg", .{
        .root_source_file = b.path("src/xberg.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    module.addLibraryPath(.{ .cwd_relative = ffi_path });
    module.addIncludePath(.{ .cwd_relative = ffi_include });
    module.linkSystemLibrary("xberg_ffi", .{});

    const test_module = b.createModule(.{
        .root_source_file = b.path("src/xberg.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    test_module.addLibraryPath(.{ .cwd_relative = ffi_path });
    test_module.addIncludePath(.{ .cwd_relative = ffi_include });
    test_module.linkSystemLibrary("xberg_ffi", .{});

    const tests = b.addTest(.{
        .root_module = test_module,
    });

    const run_tests = b.addRunArtifact(tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_tests.step);
}

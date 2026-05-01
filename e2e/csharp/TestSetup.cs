using System;
using System.IO;
using System.Runtime.CompilerServices;

namespace Kreuzberg.E2e;

/// <summary>
/// Module initializer: runs before any test.
/// </summary>
internal static class TestSetup
{
    [ModuleInitializer]
    internal static void Initialize()
    {
        // Assembly at: e2e/csharp/bin/Debug/net10.0/Kreuzberg.E2eTests.dll
        var assemblyDir = Path.GetDirectoryName(typeof(TestSetup).Assembly.Location)!;

        // Tell kreuzberg_ffi where to find libpdfium at runtime.
        // Without this the FFI calls dlopen("libpdfium.dylib") which only searches
        // system paths. Setting KREUZBERG_PDFIUM_PATH lets the Rust code bind to
        // the copy placed alongside the test assembly.
        Environment.SetEnvironmentVariable("KREUZBERG_PDFIUM_PATH", assemblyDir);

        // Change to test_documents so relative fixture paths resolve correctly.
        // repo root is five levels up from the assembly directory:
        //   net10.0 -> Debug -> bin -> csharp -> e2e -> (repo root)
        var repoRoot = Path.GetFullPath(Path.Combine(assemblyDir, "..", "..", "..", "..", ".."));
        var testDocuments = Path.Combine(repoRoot, "test_documents");
        if (Directory.Exists(testDocuments))
        {
            Directory.SetCurrentDirectory(testDocuments);
        }
    }
}

using System;
using System.Runtime.CompilerServices;
using Xunit;

// Assembly-level attributes must come first
[assembly: CollectionBehavior(DisableTestParallelization = true)]

namespace Kreuzberg.Tests;

/// <summary>
/// Module initializer that runs before any tests execute.
/// This ensures Pdfium is initialized exactly once per test run.
/// </summary>
internal static class PdfiumModuleInitializer
{
    [ModuleInitializer]
    public static void Initialize()
    {
        PdfiumInitializer.Initialize();
    }
}

/// <summary>
/// Static initializer for Pdfium library.
/// Ensures initialization happens exactly once per test run.
/// </summary>
internal static class PdfiumInitializer
{
    private static volatile bool s_initialized = false;
    private static volatile bool s_cleanupStarted = false;
    private static readonly object s_lock = new();

    public static void Initialize()
    {
        // Double-checked locking to ensure initialization happens exactly once
        if (s_initialized)
            return;

        lock (s_lock)
        {
            if (s_initialized)
                return;

            try
            {
                System.Console.WriteLine("[Test Init] Loading native library...");

                // Only load the FFI library - let Rust handle Pdfium initialization lazily
                // Pdfium will be initialized on first PDF extraction call from Rust
                NativeTestHelper.EnsureNativeLibraryLoaded();

                // NOTE: We intentionally do NOT register ProcessExit/DomainUnload handlers
                // that call into the native library. On Windows, calling FFI functions during
                // process shutdown can cause crashes because:
                // 1. The native DLL may already be partially unloaded
                // 2. The Rust runtime may be in an inconsistent state
                // 3. Thread-local storage may have been destroyed
                //
                // Instead, we rely on the Rust library to clean up its own resources when
                // the DLL is unloaded. The managed resources (GCHandlePool, InteropUtilities)
                // have their own ProcessExit handlers that clean up managed memory safely.

                System.Console.WriteLine("[Test Init] Native library loaded. Pdfium will initialize lazily on first use.");
                s_initialized = true;
            }
            catch (Exception ex)
            {
                System.Console.WriteLine($"[Test Init] Warning: {ex.Message}");
                s_initialized = true; // Mark as initialized to avoid repeated attempts
            }
        }
    }

    /// <summary>
    /// Marks that cleanup has started to prevent further FFI calls.
    /// This is called by test cleanup to signal that the process is shutting down.
    /// </summary>
    internal static void MarkCleanupStarted()
    {
        s_cleanupStarted = true;
    }

    /// <summary>
    /// Returns true if cleanup has been started and FFI calls should be avoided.
    /// </summary>
    internal static bool IsCleanupStarted => s_cleanupStarted;
}

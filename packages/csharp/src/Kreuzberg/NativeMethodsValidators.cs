using System.Runtime.InteropServices;

namespace Kreuzberg;

internal static partial class NativeMethods
{
    private const string LibNameValidators = "kreuzberg_ffi";

    [DllImport(LibNameValidators, CallingConvention = CallingConvention.Cdecl, EntryPoint = "kreuzberg_zip_bomb_validator_free")]
    internal static extern void ZipBombValidatorFree(nint ptr);
}

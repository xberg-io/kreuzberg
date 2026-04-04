// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_kreuzberg_extendr(void *dll);

// On Windows with --exclude-all-symbols, we must explicitly export the
// R package init function so R can find it when loading the DLL.
#ifdef _WIN32
__declspec(dllexport)
#endif
void R_init_kreuzberg(void *dll) {
    R_init_kreuzberg_extendr(dll);
}

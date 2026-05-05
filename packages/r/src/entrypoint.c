// Generated entrypoint: forwards to the extendr-generated init function.
// Do not edit — regenerate with `alef generate`.
#include <R_ext/Visibility.h>

void R_init_kreuzberg_extendr(void *dll);

void attribute_visible R_init_kreuzberg(void *dll) {
    R_init_kreuzberg_extendr(dll);
}

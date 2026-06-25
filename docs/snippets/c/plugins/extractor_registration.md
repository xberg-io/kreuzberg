```c title="C"
#include <xberg.h>
#include <stdio.h>

/*
 * The xberg C FFI does not expose a public function for registering
 * custom DocumentExtractor implementations from C. Document extractors must
 * be registered from Rust via `xberg::plugins::registry::get_document_extractor_registry()`
 * before the C library is loaded.
 *
 * From C you can inspect which extractors the core has registered:
 */

int main(void) {
    char *json = xberg_list_document_extractors();
    if (!json) {
        fprintf(stderr, "list document extractors failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    printf("Registered document extractors: %s\n", json);
    xberg_free_string(json);
    return 0;
}
```

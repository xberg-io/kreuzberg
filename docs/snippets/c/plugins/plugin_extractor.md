```c title="C"
#include <xberg.h>
#include <stdio.h>
#include <string.h>

/*
 * The C FFI exposes vtable-based registration for OCR backends, post-processors,
 * validators, and embedding backends. There is no public C entry point for
 * registering a custom DocumentExtractor — that must be done from Rust.
 *
 * From C you can still drive extraction for any MIME type the Rust core knows
 * how to handle. The example below feeds JSON bytes through the standard
 * extraction pipeline by passing the explicit MIME type.
 */

int main(void) {
    const char *json_payload = "{\"message\":\"Hello, world!\"}";
    const uint8_t *bytes = (const uint8_t *)json_payload;
    uintptr_t bytes_len = (uintptr_t)strlen(json_payload);

    XBERGExtractInput *input = xberg_extract_input_from_bytes(
        bytes,
        bytes_len,
        "application/json",
        "note.json"
    );

    if (!input) {
        fprintf(stderr, "input create failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    XBERGExtractionResult *result = xberg_extract(input, NULL);

    if (!result) {
        fprintf(stderr, "extraction failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_extract_input_free(input);
        return 1;
    }

    char *results = xberg_extraction_result_results(result);
    printf("Extracted JSON content: %s\n", results ? results : "(empty)");

    xberg_free_string(results);
    xberg_extraction_result_free(result);
    xberg_extract_input_free(input);
    return 0;
}
```

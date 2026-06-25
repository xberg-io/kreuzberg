```c title="C"
#include "xberg.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    /* Items is a JSON array of BatchBytesItem objects.
     * Each entry has "content" (array of byte integers), "mime_type", and an optional "config". */
    const char *items_json =
        "["
        "  {\"content\": [72,101,108,108,111,33], \"mime_type\": \"text/plain\"},"
        "  {\"content\": [87,111,114,108,100,33], \"mime_type\": \"text/plain\"}"
        "]";

    XBERGExtractionConfig *config = xberg_extraction_config_default();

    /* Returns a JSON array of ExtractionResult objects, or NULL on failure. */
    char *results_json =
        xberg_batch_extract_bytes_sync(items_json, config);
    if (!results_json) {
        fprintf(stderr, "batch extraction failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_extraction_config_free(config);
        return 1;
    }

    printf("%s\n", results_json);
    xberg_free_string(results_json);
    xberg_extraction_config_free(config);
    return 0;
}
```

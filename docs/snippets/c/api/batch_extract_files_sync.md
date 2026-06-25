```c title="C"
#include "xberg.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    /* Items is a JSON array of BatchFileItem objects.
     * Each entry has a "path" field and an optional "config" override. */
    const char *items_json =
        "["
        "  {\"path\": \"doc1.pdf\"},"
        "  {\"path\": \"doc2.docx\"},"
        "  {\"path\": \"scan.png\", \"config\": {\"force_ocr\": true}}"
        "]";

    XBERGExtractionConfig *config = xberg_extraction_config_default();

    /* Returns a JSON array of ExtractionResult objects, or NULL on failure. */
    char *results_json =
        xberg_batch_extract_files_sync(items_json, config);
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

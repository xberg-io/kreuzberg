```c title="C"
#include "xberg.h"
#include <stdio.h>

int main(void) {
    XBERGExtractInput *input = xberg_extract_input_from_uri("document.pdf");
    if (!input) {
        fprintf(stderr, "Failed to create input (code %d): %s\n",
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

    char *content = xberg_extraction_result_content(result);
    if (content) {
        printf("Content: %s\n", content);
    }
    xberg_free_string(content);

    char *metadata_json = xberg_extraction_result_metadata(result);
    if (metadata_json) {
        printf("Metadata: %s\n", metadata_json);
    }
    xberg_free_string(metadata_json);

    xberg_extract_input_free(input);
    xberg_extraction_result_free(result);
    return 0;
}
```

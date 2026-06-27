```c title="C"
#include "xberg.h"
#include <stdio.h>

int main(void) {
    const char *config_json = "{"
        "\"use_cache\": true,"
        "\"include_document_structure\": true,"
        "\"ocr\": {\"tesseract\": {\"language\": \"eng\"}}"
        "}";

    XBERGExtractionConfig *config = xberg_extraction_config_from_json(config_json);
    if (!config) {
        fprintf(stderr, "config parse failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    XBERGExtractInput *input = xberg_extract_input_from_uri("scan.pdf");
    if (!input) {
        fprintf(stderr, "Failed to create input (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_extraction_config_free(config);
        return 1;
    }

    XBERGExtractionResult *result = xberg_extract(input, config);
    if (result) {
        char *results = xberg_extraction_result_results(result);
        if (results) {
            printf("%s\n", results);
        }
        xberg_free_string(results);
    } else {
        fprintf(stderr, "extraction failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
    }

    xberg_extract_input_free(input);
    xberg_extraction_result_free(result);
    xberg_extraction_config_free(config);
    return 0;
}
```

```c title="C"
#include "xberg.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    const char *config_json =
        "{"
        "\"pages\": {"
        "\"extract_pages\": true"
        "}"
        "}";

    XBERGExtractionConfig *config = xberg_extraction_config_from_json(config_json);
    if (!config) {
        fprintf(stderr, "config parse failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    XBERGExtractInput *input = xberg_extract_input_from_uri("document.pdf");
    if (!input) {
        fprintf(stderr, "Failed to create input (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_extraction_config_free(config);
        return 1;
    }

    XBERGExtractionResult *result = xberg_extract(input, config);
    if (!result) {
        fprintf(stderr, "extraction failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_extract_input_free(input);
        xberg_extraction_config_free(config);
        return 1;
    }

    char *results = xberg_extraction_result_results(result);
    if (results) {
        printf("Extraction results (JSON): %s\n", results);
        xberg_free_string(results);
    }

    XBERGExtractionSummary *summary = xberg_extraction_result_summary(result);
    if (summary) {
        // Access summary fields via xberg_extraction_summary_* accessors
        // For page information, parse the results JSON array and access
        // individual XBERGExtractedDocument page data
        xberg_extraction_summary_free(summary);
    } else {
        printf("No extraction summary available\n");
    }

    xberg_extract_input_free(input);


    xberg_extraction_result_free(result);
    xberg_extraction_config_free(config);
    return 0;
}
```

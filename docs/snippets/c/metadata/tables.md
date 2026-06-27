```c title="C"
#include "xberg.h"
#include <stdio.h>

int main(void) {
    XBERGExtractInput *input = xberg_extract_input_from_uri("spreadsheet.xlsx");
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

    char *tables_json = xberg_extraction_result_tables(result);
    if (tables_json) {
        printf("Tables (JSON): %s\n", tables_json);
    } else {
        printf("No tables found\n");
    }
    xberg_free_string(tables_json);

    xberg_extract_input_free(input);
    xberg_extraction_result_free(result);
    return 0;
}
```

```c title="C"
#include "xberg.h"
#include <stdio.h>
#include <stdlib.h>

/* xberg_extract_file schedules work on the global Tokio runtime and
 * returns once extraction is complete.  For true non-blocking use, call it
 * from a dedicated OS thread and synchronize via a semaphore or callback. */
int main(void) {
    XBERGExtractionConfig *config = xberg_extraction_config_default();

    XBERGExtractionResult *result =
        xberg_extract_file("document.pdf", NULL, config);
    if (!result) {
        fprintf(stderr, "extraction failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_extraction_config_free(config);
        return 1;
    }

    char *content = xberg_extraction_result_content(result);
    printf("%s\n", content ? content : "(empty)");
    xberg_free_string(content);

    xberg_extraction_result_free(result);
    xberg_extraction_config_free(config);
    return 0;
}
```

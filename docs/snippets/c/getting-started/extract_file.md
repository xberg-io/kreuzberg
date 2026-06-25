```c title="C"
#include <xberg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    XBERGExtractionResult *result =
        xberg_extract_file("document.pdf", NULL, NULL);
    if (!result) {
        fprintf(stderr, "extraction failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    char *content = xberg_extraction_result_content(result);
    printf("content:\n%s\n", content ? content : "(empty)");
    xberg_free_string(content);

    char *tables_json = xberg_extraction_result_tables(result);
    printf("tables (JSON): %s\n", tables_json ? tables_json : "[]");
    xberg_free_string(tables_json);

    XBERGMetadata *metadata = xberg_extraction_result_metadata(result);
    if (metadata) {
        char *title = xberg_metadata_title(metadata);
        char *language = xberg_metadata_language(metadata);
        printf("title: %s\n", title ? title : "(none)");
        printf("language: %s\n", language ? language : "(none)");
        xberg_free_string(title);
        xberg_free_string(language);
        xberg_metadata_free(metadata);
    }

    xberg_extraction_result_free(result);
    return 0;
}
```

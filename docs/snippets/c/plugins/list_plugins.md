```c title="C"
#include <xberg.h>
#include <stdio.h>

static void print_plugin_list(const char *label, char *json) {
    if (!json) {
        fprintf(stderr, "list %s failed (code %d): %s\n",
                label,
                xberg_last_error_code(),
                xberg_last_error_context());
        return;
    }
    printf("%s: %s\n", label, json);
    xberg_free_string(json);
}

int main(void) {
    print_plugin_list("document extractors", xberg_list_document_extractors());
    print_plugin_list("OCR backends", xberg_list_ocr_backends());
    print_plugin_list("post-processors", xberg_list_post_processors());
    print_plugin_list("validators", xberg_list_validators());
    print_plugin_list("embedding presets", xberg_list_embedding_presets());
    return 0;
}
```

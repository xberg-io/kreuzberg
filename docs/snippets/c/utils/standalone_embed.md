```c title="C"
#include "xberg.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    const char *config_json =
        "{"
        "\"model\": {\"preset\": {\"name\": \"balanced\"}},"
        "\"normalize\": true"
        "}";

    XBERGEmbeddingConfig *config = xberg_embedding_config_from_json(config_json);
    if (!config) {
        fprintf(stderr, "config parse failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    /* Embed input is a JSON-encoded array of strings. */
    const char *texts_json = "[\"Hello, world!\", \"Xberg is fast\"]";

    char *embeddings_json = xberg_embed_texts(texts_json, config);
    if (!embeddings_json) {
        fprintf(stderr, "embedding failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        xberg_embedding_config_free(config);
        return 1;
    }

    printf("embeddings (JSON, 2D float array):\n%s\n", embeddings_json);
    xberg_free_string(embeddings_json);

    xberg_embedding_config_free(config);
    return 0;
}
```

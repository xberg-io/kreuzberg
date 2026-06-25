```c title="C"
#include "xberg.h"

static const char *RERANKER_CUSTOM_CONFIG_JSON =
    "{"
    "\"model\":{"
    "\"type\":\"custom\","
    "\"model_id\":\"cross-encoder/ms-marco-MiniLM-L-12-v2\","
    "\"max_length\":512"
    "},"
    "\"batch_size\":16,"
    "\"show_download_progress\":true"
    "}";
```

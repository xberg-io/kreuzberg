```c title="C"
#include "xberg.h"

static const char *RERANKER_LLM_CONFIG_JSON =
    "{"
    "\"model\":{"
    "\"type\":\"llm\","
    "\"llm\":{"
    "\"model\":\"cohere/rerank-english-v3.0\","
    "\"api_key\":\"YOUR_COHERE_API_KEY\""
    "}"
    "},"
    "\"top_k\":5,"
    "\"max_rerank_duration_secs\":30"
    "}";
```

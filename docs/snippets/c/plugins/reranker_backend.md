```c title="C"
#include <stdlib.h>
#include "xberg.h"

static xberg_Error my_reranker_rerank(
    void *backend,
    const char *query,
    const char *const *documents,
    size_t doc_count,
    float **out_scores
) {
    (void)backend;
    (void)query;
    // Return raw scores in input order; dispatcher sorts and truncates.
    float *scores = (float *)malloc(sizeof(float) * doc_count);
    if (!scores) return XBERG_ERROR_OUT_OF_MEMORY;
    for (size_t i = 0; i < doc_count; ++i) {
        scores[i] = 0.5f + (float)i * 0.1f;
    }
    *out_scores = scores;
    return XBERG_ERROR_NONE;
}

int main(void) {
    xberg_RerankerBackendVTable vt = {
        .name = "my-reranker",
        .version = "1.0.0",
        .initialize = NULL,
        .shutdown = NULL,
        .rerank = my_reranker_rerank,
    };
    xberg_register_reranker_backend(&vt, NULL);
    return 0;
}
```

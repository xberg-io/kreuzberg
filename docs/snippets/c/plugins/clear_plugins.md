```c title="C"
#include <xberg.h>
#include <stdio.h>

int main(void) {
    if (xberg_clear_post_processors() != 0) {
        fprintf(stderr, "clear post-processors failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    if (xberg_clear_ocr_backends() != 0) {
        fprintf(stderr, "clear OCR backends failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    if (xberg_clear_validators() != 0) {
        fprintf(stderr, "clear validators failed (code %d): %s\n",
                xberg_last_error_code(),
                xberg_last_error_context());
        return 1;
    }

    printf("All plugins cleared\n");
    return 0;
}
```

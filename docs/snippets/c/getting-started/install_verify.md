```c title="C"
#include <xberg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    const char *version = xberg_version();
    printf("xberg version: %s\n", version ? version : "(unknown)");
    return 0;
}
```

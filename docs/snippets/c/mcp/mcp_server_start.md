```c title="C"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

/* The xberg C FFI does not embed the MCP server. Spawn the xberg
 * CLI from a host process that uses libxberg for in-process extraction. */
int main(void) {
    pid_t pid = fork();
    if (pid < 0) {
        perror("fork");
        return 1;
    }
    if (pid == 0) {
        execlp("xberg", "xberg", "mcp", (char *)NULL);
        perror("execlp");
        _exit(127);
    }

    int status = 0;
    if (waitpid(pid, &status, 0) < 0) {
        perror("waitpid");
        return 1;
    }
    return WIFEXITED(status) ? WEXITSTATUS(status) : 1;
}
```

<!-- snippet:syntax-only --> The MCP server is exposed only through the xberg CLI; libxberg's C FFI offers no MCP entry point. This snippet spawns the CLI from a host that already links against libxberg.

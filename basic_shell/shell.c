#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

// https://github.com/spencertipping/shell-tutorial
// the tutorial contains all the stuff I would have figured out quickly
// but then stops when it gets to the parts that would be hard...
// oh, well...

int main (int argc, char** argv) {
    char*   words[256];
    char*   line        = NULL;
    size_t  line_size    = 0;
    ssize_t n;
    pid_t   child;
    int     child_status;

    while ((n = getline (&line, &line_size, stdin)) > 0) {
        // erase newline by reducing string length by one
        line[n - 1] = '\0';

        // words.split(" ")
        words[0] = line;
        for (int i = 1; (words[i] = strchr(words[i-1], ' ')); i++) {
            *(words[i]++) = '\0';
        }

        if ((child = fork ())) {
            // wait for child
            waitpid (child, &child_status, 0);
            fprintf (stderr, "Child exited with status %d\n", child_status);
            fflush (stderr);
        } else {
            // child process exec or complain
            execv (words[0], words);
            perror ("execv () failed.");
            return (1);
        }
    }
}
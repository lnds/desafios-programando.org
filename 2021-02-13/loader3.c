/** loader3.c */
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <sys/errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>

int main(int argc, char *argv[]) {
    int p[2];

    close(0);
    close(1);
    close(2);
    pipe(p);

    if (fork() == 0) {
        close(p[1]);
        int fd_stdout;
        if ((fd_stdout = open("./result.txt", O_CREAT|O_WRONLY, S_IRUSR|S_IWUSR|S_IRGRP|S_IROTH)) == -1) {
            exit(errno);
        }
        dup(fd_stdout);
        execl("./promedios", "promedios", 0);
        exit(0);
    } else {
        int i;
        close(p[0]);
        srand((unsigned int)time(NULL));
        for (i = 0; i < 30; i++) {
            float nota = 1.0 +  ((float) rand() / (float) (RAND_MAX))*6.0;
            dprintf(p[1], "%f\n", nota);
        }
        close(p[1]);
    }
}



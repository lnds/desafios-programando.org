/** loader2.c */
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <sys/errno.h>
#include <unistd.h>
#include <fcntl.h>


int main(int argc, char *argv[]) {
    int pid;
    if ((pid = fork()) == 0) {
        printf("forked");
        int fd_stdin;
        int fd_stdout;
        close(0);
        close(1);
        close(2);
        if ((fd_stdin = open("./notas.txt", O_RDONLY)) == -1) {
            exit(errno);
        }
        if ((fd_stdout = open("./result.txt", O_CREAT|O_WRONLY, S_IRUSR|S_IWUSR|S_IRGRP|S_IROTH)) == -1) {
            exit(errno);
        }
        dup(fd_stdout);
        execl("./promedios", "promedios", 0);
        exit(0);
    }

    int return_code;
    while (pid != wait(&return_code))
        ;
    printf("el proceso hijo retornó el valor: %d\n", WEXITSTATUS(return_code));
}



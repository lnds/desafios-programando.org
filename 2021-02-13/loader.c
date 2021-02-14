/** loader.c */
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    int pid;
    if ((pid = fork()) == 0) {
        execl("./promedios_v1", "promedios", 30, 0);
        exit(1);
    }

    int return_code;
    while (pid != wait(&return_code));
    printf("el proceso hijo retornó el valor: %d\n", WEXITSTATUS(return_code));
}


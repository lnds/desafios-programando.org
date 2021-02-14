#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>


int main(int argc, char *argv[]) {
    int pid;
    if ((pid = fork()) == 0) {
        int n = rand() & 0xff;
        printf("soy el proceso hijo y retornaré %d\n", n);
        exit(n);
    } else {
        int return_code;
        printf("soy el proceso padre, el pid del hijo es: %d \n", pid);
        while (pid != wait(&return_code))
            ;
        printf("el proceso hijo retornó el valor: %d\n", WEXITSTATUS(return_code));
    }
}

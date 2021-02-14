#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>


int main(int argc, char* argv[]) {
    int pid;
    if ((pid = fork()) == 0) {
        printf("soy el proceso hijo\n");
    } else {
        printf("soy el proceso padre, el pid del hijo es: %d \n", pid);
    }
    return 0;
}

/* zombi.c */
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>


int main(int argc, char* argv[]) {
    int pid;
    if ((pid = fork()) == 0) {
        printf("soy el proceso hijo\n");
        sleep(5000);
        printf("ahora soy un proceso zombi\n");
    } else {
        printf("soy el proceso padre, el pid del hijo es: %d y terminé antes\n", pid);
        //exit(0);
    }
    return 0;
}

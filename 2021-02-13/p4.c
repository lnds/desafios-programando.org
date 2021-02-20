#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>
#include <fcntl.h>

int main(int argc, char* argv[]) {
    int fd;
    char buffer[1024];
    int count;

    if ((fd = open("./notas.txt", O_RDONLY)) == -1) {
        /* no se pudo abrir el archivo */
        perror("./notas.txt");
        exit(1);
    }
    printf("fd = %d\n", fd);
    return 0;
}

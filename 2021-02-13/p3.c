#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

int bss;

int data = 0;

int main(int argc, char* argv[]) {
    int stack;
    int i;
    printf("argv[0] = %s\n", argv[0]);
    for (i = 0; i < argc; i++) {
        printf("argv[%d] = %s\n", i, argv[i]);
    }
    printf("bss = %p\n", &bss);
    printf("data = %p\n", &data);
    printf("stack = %p\n", &stack);
    return 0;
}

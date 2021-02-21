/* pthread1.c */
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

const int num_threads =  4;

void* thread_func(void *data) {
    int tid = *((int*) data);
    printf("Hola, soy el thread %d\n", tid);
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t threads[num_threads];
    int rc, i;
    int tids[num_threads];

    printf("lanzando threads...\n");
    for (i = 0; i < num_threads; i++) {
        tids[i] = i;
        rc = pthread_create(&threads[i], NULL, thread_func, (void*) &tids[i]);
    }
    printf("esperando threads...\n");
    for (i = 0; i < num_threads; i++) {
        pthread_join(threads[i], NULL);
    }
}
/* pthread3.c */
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <math.h>
#include <unistd.h>

long counter = 0;

void* tarea(void* id)
{
    int tid = (int) ((int*) id);
    printf("Hilo %d, Contador al entrar %ld \n", tid, counter);
    double result;
    for (int i = 0; i < 100000; i++)
        result += tan(i)*sin(i);
    for (int i = 0; i < 1000; i++)
        counter++;
    printf("Hilo %d, Contador al salir %ld \n", tid, counter);
    pthread_exit(0);
}

void paralelo(int tareas)
{
    int num_threads = tareas;
    pthread_t thread[num_threads];
    int rc;
    long t;
    for (t = 0; t < num_threads; t++)
    {
        printf("Creando hilo %ld\n", t);
        rc = pthread_create(&thread[t], NULL, tarea, (void *)t);
        if (rc)
        {
            printf("ERROR: codigo retornado por pthread_create() es %d\n", rc);
            exit(-1);
        }
    }
    for (t = 0; t < num_threads; t++) {
        pthread_join(thread[t], NULL);
    }

}


int main(int argc, char *argv[])
{
    if (argc != 2) {
        printf("uso: %s num_threads", argv[0]);
        exit(1);
    }
    
    int num_tasks = atoi(argv[1]);
    counter = 0;
    paralelo(num_tasks);

    printf("Proceso terminado, counter = %ld\n", counter);
    pthread_exit(NULL);
}
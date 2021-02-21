/* pthread2.c */
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <math.h>

const int num_threads = 4;

void tarea(int id)
{
    printf("Tarea %d iniciada\n", id);
    int i;
    double result = 0.0;
    for (i = 0; i < 10000000; i++)
    {
        result = result + sin(i) * tan(i);
    }
    printf("Tarea %d completada, resultado es %e\n", id, result);
}

void secuencial(int tareas)
{
    int i;
    for (i = 0; i < tareas; i++)
    {
        tarea(i);
    }
}

void *tarea_concurrente(void *t)
{
    long id = (long)t;
    printf("Hilo %ld iniciado\n", id);
    tarea(id);
    printf("Hilo %ld terminado\n", id);
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
        rc = pthread_create(&thread[t], NULL, tarea_concurrente, (void *)t);
        if (rc)
        {
            printf("ERROR: codigo retornado por pthread_create() es %d\n", rc);
            exit(-1);
        }
    }
}

void uso(int argc, char *argv[])
{
    printf("Uso: %s secuencial|paralelo num_tareas\n", argv[0]);
    exit(1);
}

int main(int argc, char *argv[])
{
    if (argc != 3)
    {
        uso(argc, argv);
    }

    int num_tasks = atoi(argv[2]);

    if (!strcmp(argv[1], "secuencial"))
    {
        secuencial(num_tasks);
    }
    else if (!strcmp(argv[1], "paralelo"))
    {
        paralelo(num_tasks);
    }
    else
    {
        uso(argc, argv);
    }

    printf("Proceso terminado\n");
    pthread_exit(NULL);
}
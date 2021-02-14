/* promedios_v2.c */
#include <stdlib.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    int i;
    int nalumnos = 0;
    double acum = 0.0;
    double promedio = 0.0;
    double nota;

    while (scanf("%lf", &nota) != EOF) {
        nalumnos++;
        acum += nota;
    }
    promedio = acum / nalumnos;
    printf("alumnos: %d\n", nalumnos);
    printf("promedio: %f\n", promedio);

    return 0;
}

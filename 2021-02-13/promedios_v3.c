/* promedios_v3.c */
#include <stdlib.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    int nalumnos = 0;
    double acum = 0.0;
    double promedio = 0.0;
    double nota;

    while (scanf("%lf", &nota) != EOF) {
        if (nota < 1.0 || nota > 7.0) {
            dprintf(2, "error la nota %lf es inválida\n", nota);
        } else {
            nalumnos++;
            acum += nota;
        }
    }
    if (nalumnos == 0) {
        dprintf(2, "error, no ha ingresado alumnos");
        exit(1);
    }
    promedio = acum / nalumnos;
    printf("alumnos: %d\n", nalumnos);
    printf("promedio: %f\n", promedio);

    return 0;
}

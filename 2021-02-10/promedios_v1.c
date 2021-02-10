/* version 1 de nuestro programa en C */
#include <stdlib.h>

int nalumnos;
double* notas;

int main(int argc, char* argv[]) {
  int i;
  double acum = 0.0;
  double promedio = 0.0;
 
  nalumnos = atoi(argv[1]);
  notas = (double *) malloc(nalumnos * sizeof(double));
  for (i = 0; i < nalumnos; i++) {
      acum += notas[i];
  }
  promedio = acum / nalumnos;
  return 0;
}

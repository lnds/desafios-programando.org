/* version 0 de nuestro programa en C */
const int nalumnos = 30;
double notas[nalumnos];

int main() {
  int i;
  double acum = 0.0;
  double promedio = 0.0;
  for (i = 0; i < nalumnos; i++) {
      acum += notas[i];
  }
  promedio = acum / nalumnos;
  return 0;
}

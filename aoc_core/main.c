#include "y2025/y2025.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void main(int argc, char *argv[]) {
  int year = 0;
  int day = 0;
  int test = 0;

  for (int i = 1; i < argc; i++) {
    if ((!strcmp(argv[i], "-y") || !strcmp(argv[i], "-Y")) && i+1 < argc) {
      year = atoi(argv[i+1]);
      i += 1;
    } else if ((!strcmp(argv[i], "-d") || !strcmp(argv[i], "-D")) && i+1 < argc) {
      day = atoi(argv[i+1]);
      i += 1;
    } else if ((!strcmp(argv[i], "-t") || !strcmp(argv[i], "-T")) && i+1 < argc) {
      test = atoi(argv[i+1]);
      i += 1;
    } else {
      printf("Unrecognized argument: %s\n", argv[i]);
    }
  }

  switch (year)
  {
  case 2025:
    y2025(day, test);
    break;
  
  default:
    break;
  }
}

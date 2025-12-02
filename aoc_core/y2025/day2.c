#include "y2025.h"

#include <stdio.h>

void y2025_day2(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day2/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day2/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  fcloase(ifp);
}

#include "y2025.h"

#include <stdio.h>

void y2025_day1(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day1/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day1/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  int cur = 50;
  int code1 = 0;
  int code2 = 0;

  char dir;
  int turn;
  while (fscanf(ifp, "%c%d\n", &dir, &turn) == 2) {
    if (dir == 'L') {
      if (turn >= cur) {
        if (cur == 0) {
          code2 -= 1;
        }
        code2 += 1 + (turn-cur)/100;
      }

      cur -= turn%100;
      if (cur < 0) {
        cur += 100;
      }
    } else if (dir == 'R') {
      if (turn >= (100 - cur)) {
        if (cur == 0 && turn < 100) {
          code2 -= 1;
        }
        code2 += 1 + (turn-(100-cur))/100;
      }

      cur += turn%100;
      if (cur > 99) {
        cur -= 100;
      }
    }

    if (cur == 0) {
      code1 += 1;
    }
  }

  printf("Year 2025 day 1 part 1: %d\n", code1);
  printf("Year 2025 day 1 part 2: %d\n", code2);

  fclose(ifp);
}

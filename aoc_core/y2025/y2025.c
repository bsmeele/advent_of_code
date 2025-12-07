#include "y2025.h"

#include <stdio.h>

void y2025(int day, int test) {
  printf("Executing year 2025, day %d, test %d\n", day, test);

  switch (day)
  {
  case 1:
    y2025_day1(test);
    break;
  case 2:
    y2025_day2(test);
    break;
  case 3:
    y2025_day3(test);
    break;
  case 4:
    y2025_day4(test);
    break;
  case 5:
    y2025_day5(test);
    break;
  case 6:
    y2025_day6(test);
    break;
  case 7:
    // y2025_day7(test);
    break;
  case 8:
    // y2025_day8(test);
    break;
  case 9:
    // y2025_day9(test);
    break;
  case 10:
    // y2025_day10(test);
    break;
  case 11:
    // y2025_day11(test);
    break;
  case 12:
    // y2025_day12(test);
    break;
  
  default:
    break;
  }
}

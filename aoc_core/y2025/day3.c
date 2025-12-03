#include "y2025.h"

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdint.h>

void y2025_day3(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day3/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day3/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  uint64_t sum1 = 0;
  uint64_t sum2 = 0;

  char line[128];
  while (fgets(line, 127, ifp) != NULL) {
    // Part 1
    int biggest = 0;
    int biggest_idx = 0;
    for (int i = 0; i < strlen(line)-2; i++) {
      if (line[i] < '0' || line[i] > '9') {
        continue;
      }
      if (line[i] - 48 > biggest) {
        biggest = line[i] - 48;
        biggest_idx = i;
      }
      if (biggest == 9) {
        break;
      }
    }

    int second = 0;
    int second_idx = 0;
    for (int i = biggest_idx + 1; i < strlen(line); i++) {
      if (line[i] < '0' || line[i] > '9') {
        continue;
      }
      if (line[i] - 48 > second) {
        second = line[i] - 48;
        second_idx = i;
      }
      if (second == 9) {
        break;
      }
    }

    sum1 += biggest * 10 + second;

    // Part 2
    uint64_t joltage[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    size_t joltage_idx[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    for (size_t i = 0; i < 12; i++) {
      size_t start = 0;
      if (i > 0) {
        start = joltage_idx[i-1] + 1;
      }
      for (size_t j = start; j < strlen(line) - (12 - i); j++) {
        if (line[i] < '0' || line[i] > '9') {
          continue;
        }
        if (line[j] - 48 > joltage[i]) {
          joltage[i] = line[j] - 48;
          joltage_idx[i] = j;
          if (joltage[i] == 9) {
            break;
          }
        }
      }
    }

    for (size_t i = 0; i < 12; i++) {
      sum2 += joltage[i] * pow(10, 11-i);
    }
    
    // printf("Line: %s", line);
    // for (size_t i = 0; i < 12; i++) {
    //   printf("%ldth biggest: %ld at %ld\n", 11-i, joltage[i], joltage_idx[i]);
    // }
    // printf("\n");
  }

  printf("Year 2025 day 3 part 1: %ld\n", sum1);
  printf("Year 2025 day 3 part 1: %ld\n", sum2);

  fclose(ifp);
}

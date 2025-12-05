#include "y2025.h"

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

void reduce_ranges(uint64_t** ranges, size_t* num_ranges) {
  for (int r = (*num_ranges)-1; r > 0; r--) {
    for (size_t r2 = 0; r2 < (*num_ranges); r2++) {
      if (r == r2) { continue; }
      if (ranges[r][0] >= ranges[r2][0] && ranges[r][1] <= ranges[r2][1]) {  // r fits completely in r2
        uint64_t* tmp = ranges[r];
        ranges[r] = ranges[(*num_ranges)-1];
        free(tmp);
        *num_ranges -= 1;
      }
      else if (ranges[r][0] >= ranges[r2][0] && ranges[r][0] <= ranges[r2][1]) {  // r starts within r2
        ranges[r2][1] = ranges[r][1];

        uint64_t* tmp = ranges[r];
        ranges[r] = ranges[(*num_ranges)-1];
        free(tmp);
        *num_ranges -= 1;
        break;
      } else if (ranges[r][1] >= ranges[r2][0] && ranges[r][1] <= ranges[r2][1]) {  // r end within r2
        ranges[r2][0] = ranges[r][0];

        uint64_t* tmp = ranges[r];
        ranges[r] = ranges[(*num_ranges)-1];
        free(tmp);
        *num_ranges -= 1;
        break;
      }
    }
  }
}

void y2025_day5(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day5/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day5/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  uint64_t** ranges = NULL;
  size_t num_ranges = 0;

  char line[64];
  while (fgets(line, 63, ifp) != NULL) {
    if (line == "\n") { break; }

    uint64_t start, end;
    if (sscanf(line, "%ld-%ld", &start, &end) != 2) { break; }

    uint64_t* range = malloc(2 * sizeof(uint64_t));
    if (range == NULL) {
      goto cleanup;
    }
    range[0] = start;
    range[1] = end;

    uint64_t** tmp = realloc(ranges, (num_ranges+1) * sizeof(uint64_t*));
    if (tmp == NULL) {
      free(range);
      goto cleanup;
    }
    ranges = tmp;

    ranges[num_ranges] = range;
    num_ranges += 1;
  }

  reduce_ranges(ranges, &num_ranges);

  uint64_t part1 = 0;
  uint64_t id;
  while (fscanf(ifp, "%ld", &id) == 1) {
    for (size_t r = 0; r < num_ranges; r++) {
      if (id >= ranges[r][0] && id <= ranges[r][1]) {
        part1 += 1;
        break;
      }
    }
  }

  uint64_t part2 = 0;
  for (size_t r = 0; r < num_ranges; r++) {
    part2 += ranges[r][1] - ranges[r][0] + 1;
  }
  
  printf("Year 2025 day 5 part 1: %ld\n", part1);
  printf("Year 2025 day 5 part 2: %ld\n", part2);

cleanup:
  for (size_t i = 0; i < num_ranges; i++) {
    free(ranges[i]);
  }
  free(ranges);
  fclose(ifp);
}

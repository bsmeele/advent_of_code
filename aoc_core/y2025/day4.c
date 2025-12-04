#include "y2025.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

int remove_rolls(char** map, int num_rows) {
  for (size_t row = 0; row < num_rows; row++) {
    for (size_t col = 0; col < strlen(map[row]); col++) {
      if (map[row][col] != '@') {
        continue;
      }

      int neighbours = 0;
      if (row > 0 && col > 0 && map[row-1][col-1] != '.') {
        neighbours += 1;
      }
      if (row > 0 && map[row-1][col] != '.') {
        neighbours += 1;
      }
      if (row > 0 && col < strlen(map[row])-1 && map[row-1][col+1] != '.') {
        neighbours += 1;
      }
      if (col < strlen(map[row])-1 && map[row][col+1] != '.') {
        neighbours += 1;
      }
      if (row < num_rows-1 && col < strlen(map[row])-1 && map[row+1][col+1] != '.') {
        neighbours += 1;
      }
      if (row < num_rows-1 && map[row+1][col] != '.') {
        neighbours += 1;
      }
      if (row < num_rows-1 && col > 0 && map[row+1][col-1] != '.') {
        neighbours += 1;
      }
      if (col > 0 && map[row][col-1] != '.') {
        neighbours += 1;
      }

      if (neighbours < 4) {
        map[row][col] = 'x';
      }
    }
  }

  int removed = 0;
  for (size_t row = 0; row < num_rows; row++) {
    for (size_t col = 0; col < strlen(map[row]); col++) {
      if (map[row][col] == 'x') {
        removed += 1;
        map[row][col] = '.';
      }
    }
  }

  return removed;
}

void y2025_day4(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day4/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day4/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  char** map = NULL;
  char line[256];
  int num_rows = 0;
  int line_size = 0;
  while (fgets(line, 256, ifp) != NULL) {
    line[strcspn(line, "\n")] = '\0';

    if (line_size == 0) {
      line_size = strlen(line);
    }
    if (strlen(line) != line_size) {
      continue;
    }

    char* row = malloc((strlen(line) + 1) * sizeof(char));
    if (row == NULL) {
      printf("Could not allocate memory\n");
      return;
    }
    strcpy(row, line);
    num_rows += 1;

    map = realloc(map, num_rows * sizeof(char*));
    if (map == NULL) {
      printf("Could not alocate memory\n");
      return;
    }
    map[num_rows-1] = row;
  }

  int part1 = remove_rolls(map, num_rows);
  int part2 = part1;
  while (1) {
    int removed = remove_rolls(map, num_rows);
    if (removed == 0) { break; }
    part2 += removed;
  }

  printf("Year 2025 day 4 part 1: %d\n", part1);
  printf("Year 2025 day 4 part 2: %d\n", part2);
}

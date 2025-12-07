#include "y2025.h"

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

void y2025_day7(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day7/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day7/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }
  
  size_t num_lines = 0;
  char** file_contents = malloc(num_lines * sizeof(char*));
  size_t buf_size = 32;
  char* line_buf = malloc(buf_size);
  size_t line_end = 0;
  if (line_buf == NULL || file_contents == NULL) {
      printf("Couldn't allocate memory for 'line_buf and/or file_contents'\n");
    goto cleanup;
  }

  while (fgets(&line_buf[line_end], buf_size - line_end, ifp) != NULL) {
    line_end = strlen(line_buf);
    if (line_buf[line_end-1] != '\n') {
      buf_size *= 2;
      char* tmp = realloc(line_buf, buf_size * sizeof(char));
      if (tmp == NULL) {
        printf("Couldn't realloate 'line_buf'\n");
        goto cleanup;
      }
      line_buf = tmp;
      continue;
    }

    char** tmp = realloc(file_contents, (num_lines + 1) * sizeof(char*));
    if (tmp == NULL) {
      printf("Couldn't reallocate file_contents\n");
      goto cleanup;
    }
    file_contents = tmp;
    file_contents[num_lines] = malloc((line_end + 1) * sizeof(char));
    if (file_contents[num_lines] == NULL) {
      printf("Couldn't allocate memory for line\n");
      goto cleanup;
    }
    strcpy(file_contents[num_lines], line_buf);
    num_lines += 1;
    line_end = 0;
  }

  size_t line_length = strlen(file_contents[0])-1;  // Exclude trailing newline

  uint64_t part1 = 0;
  uint64_t part2 = 0;

  uint64_t* row = malloc(line_length * sizeof(uint64_t));
  if (row == NULL) {
    printf("Couldn't allocate memory for row\n");
    goto cleanup;
  }
  for (size_t i = 0; i < line_length; i++) {
    row[i] = 0;
  }

  for (size_t line = 0; line < num_lines; line++) {
    for (size_t c = 0; c < line_length; c++) {
      if (file_contents[line][c] == 'S') {
        row[c] += 1;
      } else if (file_contents[line][c] == '^' && row[c] > 0) {
        part1 += 1;
        if (c > 0 && c < line_length-1) {
          row[c-1] += row[c];
          row[c+1] += row[c];
        } else {
          printf("Warning: beam split out of bounds\n");
        }
        row[c] = 0;
      }
    }
  }

  for (size_t c = 0; c < line_length; c++) {
    part2 += row[c];
  }

  printf("Year 2025 day 7 part 1: %ld\n", part1);
  printf("Year 2025 day 7 part 2: %ld\n", part2);

cleanup:
  free(line_buf);
  for (size_t r = 0; r < num_lines; r++) {
    free(file_contents[r]);
  }
  free(file_contents);

  free(row);

  fclose(ifp);
}

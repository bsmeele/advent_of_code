#include "y2025.h"

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

enum Op {
  Add,
  Mult,
};

void y2025_day6(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day6/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day6/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  size_t num_lines = 0;
  char** file_contents = malloc(num_lines * sizeof(char*));
  size_t buf_size = 64;
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

  int flag = 1;
  uint64_t part1 = 0;
  uint64_t part2 = 0;
  size_t op_idx = 0;
  while (flag) {
    enum Op op;
    while(1) {
      if (!isspace(file_contents[num_lines-1][op_idx])) {
        if (file_contents[num_lines-1][op_idx] == '\0') {
          flag = 0;
          break;
        } else if (file_contents[num_lines-1][op_idx] == '+') {
          op = Add;
        } else if (file_contents[num_lines-1][op_idx] == '*') {
          op = Mult;
        } else {
          printf("Character not regocgnized: %c\n", file_contents[num_lines-1][op_idx]);
          goto cleanup;
        }
        break;
      }

      op_idx += 1;
    }
    if (!flag) {
      break;
    }

    // ----- Part 1 -----
    uint64_t total = 0;
    if (op == Mult) {
      total = 1;
    }

    for (size_t row = 0; row < num_lines-1; row++) {
      uint64_t num;
      sscanf(file_contents[row] + op_idx, "%ld", &num);

      if (op == Add) {
        total += num;
      } else if (op == Mult) {
        total *= num;
      }
    }
    
    part1 += total;

    // ----- Part 2 -----
    total = 0;
    if (op == Mult) {
      total = 1;
    }

    int inner_flag = 1;
    size_t num_idx = 0;
    while (inner_flag) {
      inner_flag = 0;
      uint64_t num = 0;

      for (size_t row = 0; row < num_lines-1; row++) {
        if (!isspace(file_contents[row][op_idx + num_idx])) {
          num *= 10;
          num += file_contents[row][op_idx + num_idx] - 48;
          inner_flag = 1;
        }
      }

      if (inner_flag == 0) {
        break;
      }

      if (op == Add) {
        total += num;
      } else if (op == Mult) {
        total *= num;
      }

      num_idx += 1;
    }

    part2 += total;
    op_idx += 1;
  }  

  printf("Year 2025 day 6 part 1: %ld\n", part1);
  printf("Year 2025 day 6 part 2: %ld\n", part2);

cleanup:
  free(line_buf);
  for (size_t r = 0; r < num_lines; r++) {
    free(file_contents[r]);
  }
  free(file_contents);

  fclose(ifp);
}

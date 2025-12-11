#include "y2025.h"

#include "utils/vector/vec_u64.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

struct Node {
  char name[4];
  struct Vec from;
};

uint64_t get_all_paths(struct Node* nodes, size_t num_nodes, size_t start_idx, size_t end_idx, uint64_t* cache) {
  if (start_idx >= num_nodes || end_idx >= num_nodes) {
    printf("start and/or end index not in range\n");
    return 0;
  }

  if (cache[end_idx] != -1) {
    return cache[end_idx];
  }
  
  if (start_idx == end_idx) {
    return 1;
  }
  
  uint64_t total = 0;
  for (size_t i = 0; i < nodes[end_idx].from.size; i++) {
    total += get_all_paths(nodes, num_nodes, start_idx, nodes[end_idx].from.mem[i], cache);
  }
  
  cache[end_idx] = total;

  return total;
}

void clear_cache(uint64_t* cache, size_t n) {
  for (size_t i = 0; i < n; i++) {
    cache[i] = -1;
  }
}

void y2025_day11(int test) {
  FILE* ifp;
  if (test == 1) {
    ifp = fopen("../input/y2025/day11/test.txt", "r");
  } else if (test == 2) {
    ifp = fopen("../input/y2025/day11/test2.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day11/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  uint64_t part1 = 0;
  uint64_t part2 = 0;

  struct Node* nodes = NULL;
  size_t num_nodes = 0;

  size_t you_idx = UINT64_MAX;
  size_t out_idx = UINT64_MAX;
  size_t svr_idx = UINT64_MAX;
  size_t dac_idx = UINT64_MAX;
  size_t fft_idx = UINT64_MAX;

  size_t buf_size = 32;
  char* line_buf = malloc(buf_size);
  size_t line_end = 0;
  if (line_buf == NULL) {
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

    char name[4];
    if (sscanf(line_buf, "%3[^:]", name) != 1) {
      printf("Couldn't find node name\n");
      goto cleanup;
    }

    if (strcmp(name, "you") == 0) {
      you_idx = num_nodes;
    } else if (strcmp(name, "svr") == 0) {
      svr_idx = num_nodes;
    } else if (strcmp(name, "dac") == 0) {
      dac_idx = num_nodes;
    } else if (strcmp(name, "fft") == 0) {
      fft_idx = num_nodes;
    }

    struct Node* tmp = realloc(nodes, (num_nodes+1) * sizeof(struct Node));
    if (tmp == NULL) {
      printf("Couln't realloc nodes\n");
      goto cleanup;
    }
    nodes = tmp;

    struct Node node;
    strcpy(node.name, name);
    node.from = vec_new();
    nodes[num_nodes] = node;
    num_nodes += 1;

    line_end = 0;
  }

  out_idx = num_nodes;

  struct Node* tmp = realloc(nodes, (num_nodes+1) * sizeof(struct Node));
  if (tmp == NULL) {
    printf("Couln't realloc nodes\n");
    goto cleanup;
  }
  nodes = tmp;

  struct Node node;
  strcpy(node.name, "out");
  node.from = vec_new();
  nodes[num_nodes] = node;
  num_nodes += 1;

  size_t line = 0;
  rewind(ifp);
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
    line_buf[line_end-1] = '\0';
    line_end -= 1;

    char* tok = strtok(line_buf + 4, " ");
    while (tok != NULL) {
      for (size_t n = 0; n < num_nodes; n++) {
        if (strcmp(nodes[n].name, tok) == 0) {
          vec_push(&nodes[n].from, line);
          break;
        }
      }
      tok = strtok(NULL, " ");
    }

    if (line >= num_nodes) {
      printf("more lines than nodes\n");
      break;
    }
    line += 1;

    line_end = 0;
  }

  uint64_t* cache = malloc(num_nodes * sizeof(uint64_t));
  if (cache == NULL) {
    printf("Couldn't allocate space for cache\n");
    goto cleanup;
  }
  clear_cache(cache, num_nodes);

  part1 = get_all_paths(nodes, num_nodes, you_idx, out_idx, cache);

  printf("Year 2025 day 11 part 1: %ld\n", part1);

  clear_cache(cache, num_nodes);
  uint64_t dac_fft = get_all_paths(nodes, num_nodes, dac_idx, fft_idx, cache);

  clear_cache(cache, num_nodes);
  uint64_t fft_dac = get_all_paths(nodes, num_nodes, fft_idx, dac_idx, cache);
  
  if (dac_fft != 0) {
    clear_cache(cache, num_nodes);
    uint64_t svr_dac = get_all_paths(nodes, num_nodes, svr_idx, dac_idx, cache);

    clear_cache(cache, num_nodes);
    uint64_t fft_out = get_all_paths(nodes, num_nodes, fft_idx, out_idx, cache);

    part2 = svr_dac * dac_fft * fft_out;

  } else {
    clear_cache(cache, num_nodes);
    uint64_t svr_fft = get_all_paths(nodes, num_nodes, svr_idx, fft_idx, cache);

    clear_cache(cache, num_nodes);
    uint64_t dac_out = get_all_paths(nodes, num_nodes, dac_idx, out_idx, cache);

    part2 = svr_fft * fft_dac * dac_out;
  }

  printf("Year 2025 day 11 part 2: %ld\n", part2);

cleanup:
  for (size_t n = 0; n < num_nodes; n++) {
    vec_free(&nodes[n].from);
  }
  free(nodes);
  free(cache);

  free(line_buf);
  fclose(ifp);
}

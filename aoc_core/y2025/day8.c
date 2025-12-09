#include "y2025.h"

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

struct Vec {
  uint64_t* mem;
  size_t size;
  size_t capacity;
};

struct Vec vec_new() {
  struct Vec vec;
  vec.size = 0;
  vec.capacity = 8;
  vec.mem = malloc(vec.capacity * sizeof(uint64_t));
  return vec;
}

struct Vec vec_new_with_capacity(size_t capacity) {
  struct Vec vec;
  vec.size = 0;
  vec.capacity = capacity;
  vec.mem = malloc(vec.capacity * sizeof(uint64_t));
  if (vec.mem == NULL) {
    printf("Couldn't allocate memory for vec\n");
  }
  return vec;
}

void vec_push(struct Vec* vec, uint64_t data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    uint64_t* tmp = realloc(vec->mem, vec->capacity * sizeof(uint64_t));
    if (tmp == NULL) {
      printf("Couln't realloc vec\n");
      return;
    }
    vec->mem = tmp;
  }

  vec->mem[vec->size] = data;
  vec->size += 1;
}

uint64_t vec_pop(struct Vec* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return 0;
  }

  vec->size -= 1;
  return vec->mem[vec->size];
}

uint64_t vec_peak(struct Vec* vec, size_t idx) {
  if (idx >= vec->size) {
    printf("Index out of bounds\n");
    return 0;
  }

  return vec->mem[idx];
}

void vec_clear(struct Vec* vec) {
  vec->size = 0;
}

void vec_free(struct Vec* vec) {
  free(vec->mem);
}

void y2025_day8(int test) {
  uint64_t cables;
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day8/test.txt", "r");
    cables = 10;
  } else {
    ifp = fopen("../input/y2025/day8/input.txt", "r");
    cables = 1000;
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  uint64_t part1 = 0;
  uint64_t part2 = 0;
  
  size_t num_rows = 0;
  uint64_t** boxes = malloc(num_rows * sizeof(uint64_t*));

  char linebuf[32];
  while (fgets(linebuf, 32, ifp) != NULL) {
    uint64_t* coords = malloc(3 * sizeof(uint64_t));
    if (sscanf(linebuf, "%ld, %ld, %ld", &coords[0], &coords[1], &coords[2]) != 3) {
      free(coords);
      break;
    }

    uint64_t** tmp = realloc(boxes, (num_rows + 1) * sizeof(uint64_t*));
    if (tmp == NULL) {
      printf("Couldn't reallocate boxes\n");
      free(coords);
      goto cleanup;
    }
    boxes = tmp;

    boxes[num_rows] = coords;
    num_rows += 1;
  }

  size_t num_dists = (num_rows * (num_rows-1))/2;
  uint64_t* dists = malloc(num_dists * 3 * sizeof(uint64_t));
  if (dists == NULL) {
    printf("Couldn't allocate memory for dists\n");
    goto cleanup;
  }
  for (size_t i = 0; i < num_dists; i++) {
    dists[i*3] = UINT64_MAX;
    dists[i*3 + 1] = 0;
    dists[i*3 + 2] = 0;
  }
  size_t dists_len = 0;

  // Calculate all distance permutations
  for (size_t box1 = 0; box1 < num_rows; box1++) {
    for (size_t box2 = box1 + 1; box2 < num_rows; box2++) {
      uint64_t dist = 0;
      dist += (boxes[box1][0] - boxes[box2][0]) * (boxes[box1][0] - boxes[box2][0]);
      dist += (boxes[box1][1] - boxes[box2][1]) * (boxes[box1][1] - boxes[box2][1]);
      dist += (boxes[box1][2] - boxes[box2][2]) * (boxes[box1][2] - boxes[box2][2]);

      dists[3*dists_len] = dist;
      dists[3*dists_len + 1] = box1;
      dists[3*dists_len + 2] = box2;
      dists_len += 1;
    }
  }

  uint64_t* labels = malloc(num_rows * sizeof(uint64_t));
  if (labels == NULL) {
    printf("Couldn't allocate memory for labels\n");
    goto cleanup;
  }
  for (size_t i = 0; i < num_rows; i++) {
    labels[i] = 0;
  }
  struct Vec stack = vec_new();
  uint64_t* sorted_dists = malloc(num_dists * sizeof(uint64_t));
  if (sorted_dists == NULL) {
    printf("Couldn't allocate memory for sorted dists\n");
    goto cleanup;
  }
  size_t sorted_dists_len = 0;

  for (size_t num_cons = 1; num_cons <= num_dists; num_cons++) {

    // Find the shorted distance
    uint64_t shorted_dist = UINT64_MAX;
    size_t shorted_dist_idx = 0;
    // Find shortest dist
    for (size_t dist = 0; dist < num_dists; dist++) {
      if (dists[3*dist] < shorted_dist) {
        shorted_dist = dists[3*dist];
        shorted_dist_idx = dist;
      }
    }
    // Store the shortest distance in a sorted list and mark the entry as sorted
    sorted_dists[sorted_dists_len] = shorted_dist_idx;
    sorted_dists_len += 1;
    dists[3*shorted_dist_idx] = UINT64_MAX;

    // Update groupings
    if (labels[dists[3*shorted_dist_idx + 1]] == 0) {
      labels[dists[3*shorted_dist_idx + 1]] = num_cons;
    }
    labels[dists[3*shorted_dist_idx + 2]] = labels[dists[3*shorted_dist_idx + 1]];
    vec_push(&stack, dists[3*shorted_dist_idx + 2]);

    while (stack.size > 0) {
      size_t cur = vec_pop(&stack);

      for (size_t con = 0; con < sorted_dists_len; con++) {
        size_t con_idx = sorted_dists[con];

        if (dists[3*con_idx + 1] == cur && labels[dists[3*con_idx + 2]] != labels[cur]) {
          labels[dists[3*con_idx + 2]] = labels[cur];
          vec_push(&stack, dists[3*con_idx + 2]);
        } else if (dists[3*con_idx + 2] == cur && labels[dists[3*con_idx + 1]] != labels[cur]) {
          labels[dists[3*con_idx + 1]] = labels[cur];
          vec_push(&stack, dists[3*con_idx + 1]);
        }
      }
    }
    vec_clear(&stack);

    // Part 1
    if (num_cons == cables) {
      uint64_t largest[3];
      largest[0] = 0;
      largest[1] = 0;
      largest[2] = 0;

      for (size_t label = 1; label <= num_rows; label++) {
        size_t group_size = 0;
        for (size_t idx = 0; idx < num_rows; idx++) {
          if (labels[idx] == label) {
            group_size += 1;
          }
        }
      
        if (group_size > largest[0]) {
          largest[2] = largest[1];
          largest[1] = largest[0];
          largest[0] = group_size;
        } else if (group_size > largest[1]) {
          largest[2] = largest[1];
          largest[1] = group_size;
        } else if (group_size > largest[2]) {
          largest[2] = group_size;
        }
      }

      part1 = largest[2] * largest[1] * largest[0];
    }

    // Part 2
    int flag = 1;
    uint64_t label = labels[0];
    for (size_t idx = 0; idx < num_rows; idx++) {
      if (labels[idx] != label) {
        flag = 0;
        break;
      }
    }
    if (flag) {
      part2 = boxes[dists[3*shorted_dist_idx + 1]][0] * boxes[dists[3*shorted_dist_idx + 2]][0];
      break;
    }
  }

  printf("Year 2025 day 8 part 1: %ld\n", part1);
  printf("Year 2025 day 8 part 2: %ld\n", part2);

cleanup:
  for (size_t r = 0; r < num_rows; r++) {
    free(boxes[r]);
  }
  free(boxes);
  free(dists);
  free(labels);
  vec_free(&stack);
  free(sorted_dists);

  fclose(ifp);
}

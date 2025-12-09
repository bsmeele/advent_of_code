#include "y2025.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int point_in_shape(uint64_t** tiles, size_t num_tiles, uint64_t x, uint64_t y) {
  uint64_t crosssings = 0;
  for (size_t t1 = 0; t1 < num_tiles; t1++) {
    uint64_t t2 = (t1+1)%num_tiles;

    uint64_t t1x = tiles[t1][0];
    uint64_t t1y = tiles[t1][1];
    uint64_t t2x = tiles[t2][0];
    uint64_t t2y = tiles[t2][1];

    if (t1x > t2x) {
      uint64_t tmp = t1x;
      t1x = t2x;
      t2x = tmp;
    }
    if (t1y > t2y) {
      uint64_t tmp = t1y;
      t1y = t2y;
      t2y = tmp;
    }

    if ((t1x == x && t1y == y) || (t2x == x && t2y == y)) { return 1; }
    if (t1x > x) { continue; }
    if (t1x == x && t2x == x && t1y < y && t2y > y) {
      // On a vertical edge
      return 1;
    }
    if (t1y == y && t2y == y && t2x > x) {
      // On a horizontal edge
      return 1;
    }
    if ((t1y > y && t2y <= y)
    || (t1y < y && t2y >= y)) {
      // Vertical edge crossing
      crosssings += 1;
    }
  }
  return crosssings%2 == 1;
}

int edge_intersect_polygon(uint64_t** tiles, size_t num_tiles, uint64_t x1, uint64_t y1, uint64_t x2, uint64_t y2) {
  if (x1 > x2) {
    uint64_t tmp = x1;
    x1 = x2;
    x2 = tmp;
  }
  if (y1 > y2) {
    uint64_t tmp = y1;
    y1 = y2;
    y2 = tmp;
  }

  for (size_t t1 = 0; t1 < num_tiles; t1++) {
    size_t t2 = (t1+1)%num_tiles;

    uint64_t t1x = tiles[t1][0];
    uint64_t t1y = tiles[t1][1];
    uint64_t t2x = tiles[t2][0];
    uint64_t t2y = tiles[t2][1];

    if (t1x > t2x) {
      uint64_t tmp = t1x;
      t1x = t2x;
      t2x = tmp;
    }
    if (t1y > t2y) {
      uint64_t tmp = t1y;
      t1y = t2y;
      t2y = tmp;
    }

    if (x1 == x2 && t1y == t2y
      && t1x < x1 && t2x > x1
      && t1y > y1 && t1y < y2) {
      return 1;
    } else if (y1 == y2 && t1x == t2x
      && t1x > x1 && t1x < x2
      && t1y < y1 && t2y > t2y) {
      return 1;
    }
  }

  return 0;
}

void y2025_day9(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day9/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day9/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  uint64_t part1 = 0;
  uint64_t part2 = 0;
  
  size_t num_tiles = 0;
  uint64_t** tiles = malloc(num_tiles * sizeof(uint64_t*));

  char linebuf[32];
  while (fgets(linebuf, 32, ifp) != NULL) {
    uint64_t* coords = malloc(2 * sizeof(uint64_t));
    if (sscanf(linebuf, "%ld, %ld", &coords[0], &coords[1]) != 2) {
      free(coords);
      break;
    }

    uint64_t** tmp = realloc(tiles, (num_tiles + 1) * sizeof(uint64_t*));
    if (tmp == NULL) {
      printf("Couldn't reallocate boxes\n");
      free(coords);
      goto cleanup;
    }
    tiles = tmp;

    tiles[num_tiles] = coords;
    num_tiles += 1;
  }

  for (size_t t1 = 0; t1 < num_tiles-1; t1++) {
    for (size_t t2 = t1+1; t2 < num_tiles; t2++) {
      uint64_t left_x = tiles[t1][0];
      uint64_t top_y = tiles[t1][1];
      uint64_t right_x = tiles[t2][0];
      uint64_t bottom_y = tiles[t2][1];
      if (left_x > right_x) {
        uint64_t tmp = left_x;
        left_x = right_x;
        right_x = tmp;
      }
      if (top_y > bottom_y) {
        uint64_t tmp = top_y;
        top_y = bottom_y;
        bottom_y = tmp;
      }

      uint64_t dx = right_x - left_x + 1;
      uint64_t dy = bottom_y - top_y + 1;
      uint64_t size = dx * dy;
      if (size > part1) {
        part1 = size;
      }

      int flag = 1;

      // Check if corners of the rectangle are in the shape
      if (!point_in_shape(tiles, num_tiles, left_x, top_y)
      || !point_in_shape(tiles, num_tiles, right_x, top_y)
      || !point_in_shape(tiles, num_tiles, right_x, bottom_y)
      || !point_in_shape(tiles, num_tiles, left_x, bottom_y)
      || edge_intersect_polygon(tiles, num_tiles, left_x, top_y, right_x, top_y)
      || edge_intersect_polygon(tiles, num_tiles, right_x, top_y, right_x, bottom_y)
      || edge_intersect_polygon(tiles, num_tiles, right_x, bottom_y, left_x, bottom_y)
      || edge_intersect_polygon(tiles, num_tiles, left_x, bottom_y, left_x, top_y)) {
        flag = 0;
      }

      if (flag && size > part2) {
        part2 = size;
      }
    }
  }

  printf("Year 2025 day 8 part 1: %ld\n", part1);
  printf("Year 2025 day 8 part 2: %ld\n", part2);

cleanup:
  for (size_t r = 0; r < num_tiles; r++) {
    free(tiles[r]);
  }
  free(tiles);

  fclose(ifp);
}

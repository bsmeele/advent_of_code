#include "y2025.h"

#include <stdio.h>
#include <string.h>
#include <stdint.h>

void y2025_day2(int test) {
  FILE* ifp;
  if (test) {
    ifp = fopen("../input/y2025/day2/test.txt", "r");
  } else {
    ifp = fopen("../input/y2025/day2/input.txt", "r");
  }

  if (ifp == NULL) {
    printf("Failed to open file\n");
    return;
  }

  uint64_t sum1 = 0;
  uint64_t sum2 = 0;

  uint64_t id1;
  uint64_t id2;
  while (fscanf(ifp, "%ld-%ld%*[,\n]", &id1, &id2) == 2) {
    for (uint64_t id = id1; id <= id2; id++) {
      char id_str[16];
      sprintf(id_str, "%ld", id);

      for (size_t sub_len = strlen(id_str)/2; sub_len > 0; sub_len--) {
        if (strlen(id_str)%sub_len != 0) {
          continue;
        }
        
        int flag = 1;
        char* p1 = id_str;
        for (int i = 1; i < strlen(id_str)/sub_len; i++) {
          if (strncmp(p1, p1 + sub_len, sub_len) != 0) {
            flag = 0;
            break;
          }
          p1 += sub_len;
        }

        if (flag) {
          sum2 += id;
          if (strlen(id_str)%2 == 0 && sub_len == strlen(id_str)/2) {
            sum1 += id;
          }
          break;
        }
      }
    }
  }

  printf("Year 2025 day 2 part 1: %ld\n", sum1);
  printf("Year 2025 day 2 part 1: %ld\n", sum2);

  fclose(ifp);
}

#include "vec_str.h"

#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

static const size_t MAX_VEC_SIZE = 1 << 16;

struct VecStr vec_str_new() {
  struct VecStr vec;
  vec.size = 0;
  vec.capacity = 8;
  vec.mem = malloc(vec.capacity * sizeof(char*));
  return vec;
}

struct VecStr vec_str_new_with_capacity(size_t capacity) {
  assert(capacity < MAX_VEC_SIZE);
  struct VecStr vec;
  vec.size = 0;
  vec.capacity = capacity;
  vec.mem = malloc(vec.capacity * sizeof(char*));
  if (vec.mem == NULL) {
    printf("Couldn't allocate memory for vec\n");
  }
  return vec;
}

void vec_str_push(struct VecStr* vec, char* data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
    char** tmp = realloc(vec->mem, vec->capacity * sizeof(char*));
    if (tmp == NULL) {
      printf("Couln't realloc vec\n");
      return;
    }
    vec->mem = tmp;
  }

  vec->mem[vec->size] = data;
  vec->size += 1;
}

void vec_str_push_front(struct VecStr* vec, char* data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
    char** tmp = realloc(vec->mem, vec->capacity * sizeof(char*));
    if (tmp == NULL) {
      printf("Couln't realloc vec\n");
      return;
    }
    vec->mem = tmp;
  }

  vec->size += 1;
  for (size_t s = vec->size-1; s > 0; s++) {
    vec->mem[s] = vec->mem[s-1];
  }

  vec->mem[0] = data;
}

char* vec_str_pop(struct VecStr* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return 0;
  }

  vec->size -= 1;
  return vec->mem[vec->size];
}

char* vec_str_pop_front(struct VecStr* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return 0;
  }

  char* data = vec->mem[0];

  vec->size -= 1;
  for (size_t s = 0; s < vec->size; s++) {
    vec->mem[s] = vec->mem[s+1];
  }

  return data;
}

char* vec_str_peak(struct VecStr* vec, size_t idx) {
  if (idx >= vec->size) {
    printf("Index out of bounds\n");
    return 0;
  }

  return vec->mem[idx];
}

void vec_str_clear(struct VecStr* vec) {
  vec->size = 0;
}

void vec_str_free(struct VecStr* vec) {
  for (size_t i = 0; i < vec->size; i++) {
    free(vec->mem[i]);
  }
  free(vec->mem);
}

#include "vec_vec_u64.h"

#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

static const size_t MAX_VEC_SIZE = 1 << 16;

struct VecVec vec_vec_new() {
  struct VecVec vec;
  vec.size = 0;
  vec.capacity = 8;
  vec.mem = malloc(vec.capacity * sizeof(struct Vec));
  return vec;
}

struct VecVec vec_vec_new_with_capacity(size_t capacity) {
  assert(capacity < MAX_VEC_SIZE);
  struct VecVec vec;
  vec.size = 0;
  vec.capacity = capacity;
  vec.mem = malloc(vec.capacity * sizeof(struct Vec));
  if (vec.mem == NULL) {
    printf("Couldn't allocate memory for vec\n");
  }
  return vec;
}

void vec_vec_push(struct VecVec* vec, struct Vec data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
    struct Vec* tmp = realloc(vec->mem, vec->capacity * sizeof(struct Vec));
    if (tmp == NULL) {
      printf("Couln't realloc vec\n");
      return;
    }
    vec->mem = tmp;
  }

  vec->mem[vec->size] = data;
  vec->size += 1;
}

void vec_vec_push_front(struct VecVec* vec, struct Vec data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
    struct Vec* tmp = realloc(vec->mem, vec->capacity * sizeof(struct Vec));
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

struct Vec vec_vec_pop(struct VecVec* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return vec_new_with_capacity(0);
  }

  vec->size -= 1;
  return vec->mem[vec->size];
}

struct Vec vec_vec_pop_front(struct VecVec* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return vec_new_with_capacity(0);
  }

  struct Vec data = vec->mem[0];

  vec->size -= 1;
  for (size_t s = 0; s < vec->size; s++) {
    vec->mem[s] = vec->mem[s+1];
  }

  return data;
}

struct Vec vec_vec_peak(struct VecVec* vec, size_t idx) {
  if (idx >= vec->size) {
    printf("Index out of bounds\n");
    return vec_new_with_capacity(0);
  }

  return vec->mem[idx];
}

void vec_vec_clear(struct VecVec* vec) {
  vec->size = 0;
}

void vec_vec_free(struct VecVec* vec) {
  for (size_t i = 0; i < vec->size; i++) {
    vec_free(&vec->mem[i]);
  }
  free(vec->mem);
}

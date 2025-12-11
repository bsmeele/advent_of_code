#include "vec_u64.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

static const size_t MAX_VEC_SIZE = 1 << 16;

struct Vec vec_new() {
  struct Vec vec;
  vec.size = 0;
  vec.capacity = 8;
  vec.mem = malloc(vec.capacity * sizeof(uint64_t));
  return vec;
}

struct Vec vec_new_with_capacity(size_t capacity) {
  assert(capacity < MAX_VEC_SIZE);

  struct Vec vec;
  vec.size = 0;
  vec.capacity = capacity;
  vec.mem = malloc(vec.capacity * sizeof(uint64_t));
  if (vec.mem == NULL) {
    printf("Couldn't allocate memory for vec\n");
  }
  return vec;
}

struct Vec vec_new_from_vec(struct Vec* vec) {
  struct Vec new;
  new.size = vec->size;
  new.capacity = vec->capacity;
  new.mem = malloc(vec->capacity * sizeof(uint64_t));
  if (new.mem == NULL) {
    printf("Couldn't allocate memory for vec\n");
  }
  memcpy(new.mem, vec->mem, vec->size * sizeof(uint64_t));
  return new;
}

void vec_push(struct Vec* vec, uint64_t data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
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

void vec_push_front(struct Vec* vec, uint64_t data) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
    uint64_t* tmp = realloc(vec->mem, vec->capacity * sizeof(uint64_t));
    if (tmp == NULL) {
      printf("Couln't realloc vec\n");
      return;
    }
    vec->mem = tmp;
  }

  vec->size += 1;
  for (size_t s = vec->size-1; s > 0; s--) {
    vec->mem[s] = vec->mem[s-1];
  }

  vec->mem[0] = data;
}

void vec_insert(struct Vec* vec, uint64_t data, size_t idx) {
  if (vec->size == vec->capacity) {
    vec->capacity *= 2;
    assert(vec->capacity < MAX_VEC_SIZE);
    uint64_t* tmp = realloc(vec->mem, vec->capacity * sizeof(uint64_t));
    if (tmp == NULL) {
      printf("Couln't realloc vec\n");
      return;
    }
    vec->mem = tmp;
  }

  vec->size += 1;
  for (size_t i = vec->size-1; i > idx; i--) {
    vec->mem[i] = vec->mem[i-1];
  }

  vec->mem[idx] = data;
}

uint64_t vec_pop(struct Vec* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return 0;
  }

  vec->size -= 1;
  return vec->mem[vec->size];
}

uint64_t vec_pop_front(struct Vec* vec) {
  if (vec->size == 0) {
    printf("Can not pop from empty vec\n");
    return 0;
  }

  uint64_t data = vec->mem[0];

  vec->size -= 1;
  for (size_t s = 0; s < vec->size; s++) {
    vec->mem[s] = vec->mem[s+1];
  }

  return data;
}

uint64_t vec_remove(struct Vec* vec, size_t idx) {
  if (vec->size == 0) {
    printf("Can not remove from empty vec\n");
    return 0;
  }

  uint64_t data = vec->mem[idx];

  vec->size -= 1;
  for (size_t i = idx; i < vec->size; i++) {
    vec->mem[i] = vec->mem[i+1];
  }

  return data;
}

uint64_t vec_peak(struct Vec* vec, size_t idx) {
  if (idx >= vec->size) {
    printf("Index out of bounds\n");
    return 0;
  }

  return vec->mem[idx];
}

uint64_t vec_eq(struct Vec* a, struct Vec* b) {
  if (a->size != b->size) {
    return 0;
  }
  for (size_t i = 0; i < a->size; i++) {
    if (a->mem[i] != b->mem[i]) {
      return 0;
    }
  }
  return 1;
}

void vec_clear(struct Vec* vec) {
  vec->size = 0;
}

void vec_free(struct Vec* vec) {
  free(vec->mem);
}

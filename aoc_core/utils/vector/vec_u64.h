#ifndef VEC_U64_H
#define VEC_U64_H

#include <stdint.h>
#include <stddef.h>

struct Vec {
  uint64_t* mem;
  size_t size;
  size_t capacity;
};

struct Vec vec_new();
struct Vec vec_new_with_capacity(size_t capacity);
struct Vec vec_new_from_vec(struct Vec* vec);
void vec_push(struct Vec* vec, uint64_t data);
void vec_push_front(struct Vec* vec, uint64_t data);
void vec_insert(struct Vec* vec, uint64_t data, size_t idx);
uint64_t vec_pop(struct Vec* vec);
uint64_t vec_pop_front(struct Vec* vec);
uint64_t vec_remove(struct Vec* vec, size_t idx);
uint64_t vec_peak(struct Vec* vec, size_t idx);
uint64_t vec_eq(struct Vec* a, struct Vec* b);
void vec_clear(struct Vec* vec);
void vec_free(struct Vec* vec);

#endif  // VEC_U64_H

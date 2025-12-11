#ifndef VEC_VEC_U64_H
#define VEC_VEC_U64_H

#include "vec_u64.h"

#include <stdint.h>
#include <stddef.h>

struct VecVec {
  struct Vec* mem;
  size_t size;
  size_t capacity;
};

struct VecVec vec_vec_new();
struct VecVec vec_vec_new_with_capacity(size_t capacity);
void vec_vec_push(struct VecVec* vec, struct Vec data);
void vec_vec_push_front(struct VecVec* vec, struct Vec data);
struct Vec vec_vec_pop(struct VecVec* vec);
struct Vec vec_vec_pop_front(struct VecVec* vec);
struct Vec vec_vec_peak(struct VecVec* vec, size_t idx);
void vec_vec_clear(struct VecVec* vec);
void vec_vec_free(struct VecVec* vec);

#endif  // VEC_VEC_U64_H

#ifndef VEC_STR_H
#define VEC_STR_H

#include <stdint.h>
#include <stddef.h>

struct VecStr {
  char** mem;
  size_t size;
  size_t capacity;
};

struct VecStr vec_str_new();
struct VecStr vec_str_new_with_capacity(size_t capacity);
void vec_str_push(struct VecStr* vec, char* data);
void vec_str_push_front(struct VecStr* vec, char* data);
char* vec_str_pop(struct VecStr* vec);
char* vec_str_pop_front(struct VecStr* vec);
char* vec_str_peak(struct VecStr* vec, size_t idx);
void vec_str_clear(struct VecStr* vec);
void vec_str_free(struct VecStr* vec);

#endif  // VEC_STR_H

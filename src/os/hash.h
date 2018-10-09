#ifndef _GLR_HASH_H
#define _GLR_HASH_H

#include "sys.h"

typedef uint32_t glr_hash_t;

#define glr_hash_uptr(uptr) glr_hash_ptr((void*) (uptr))

#define glr_hash_ptr(ptr) glr_hash_u32((const uint32_t)(((uintptr_t) (ptr)) >> 3))

glr_hash_t glr_hash_u32(uint32_t value);

glr_hash_t glr_hash_string(const char* str, size_t len);

#endif // _GLR_HASH_H
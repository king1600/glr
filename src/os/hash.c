#include "hash.h"

#define FNV_PRIME  0x1000193
#define FNV_OFFSET 0x811c9dc5

glr_hash_t glr_hash_string(const char* str, size_t len) {
    glr_hash_t hash = FNV_OFFSET;
    while (len--)
        hash = (hash ^ (*str++)) * FNV_PRIME;
    return hash;
}

glr_hash_t glr_hash_u32(uint32_t value) {
    value = ~value + (value << 15);
    value = value ^ (value >> 12);
    value = value + (value << 2);
    value = value ^ (value >> 4);
    value = value * 2057;
    value = value ^ (value >> 16);
    return value;
}
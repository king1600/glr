#ifndef _GLR_CONSTPOOL_H
#define _GLR_CONSTPOOL_H

#include "../../os/sys.h"

typedef struct {
    uintptr_t end;
    union {
        uint8_t* u8;
        uint16_t* u16;
        uint32_t* u32;
        uint64_t* u64;
        int32_t* i32;
        int64_t* i64;
        float* f32;
        double* f64;
    } pos;
} glr_reader_t;

#define GLR_READ_LEN(reader) \
    (((uint8_t*) (reader).end) - ((reader).pos.u8))
#define GLR_READ(reader, field, var, error)                    \
    if (GLR_READ_LEN(reader) < sizeof(reader.pos.field)) \
        return error;                                          \
    var = *reader.pos.field++;

#define GLR_CONST_I64 0
#define GLR_CONST_U64 1
#define GLR_CONST_F64 2
#define GLR_CONST_STR 3

typedef struct {
    char* text;
    size_t size;
} glr_string_t;

typedef struct {
    uint8_t type;
    union {
        double f64;
        int64_t i64;
        uint64_t u64;
        glr_string_t string;
    } data;
} glr_const_t;

typedef struct {
    size_t size;
    glr_const_t* pool;
} glr_const_pool_t;

uintptr_t glr_const_pool_load(glr_reader_t* reader);

#endif // _GLR_CONSTPOOL_H
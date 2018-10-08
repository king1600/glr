#include "classfile.h"

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

glr_class_result glr_class_load(glr_vm_t* vm, uint8_t* bytes, size_t size) {
    return GLR_CLASS_ERROR;
}
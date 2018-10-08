#ifndef _GLR_CLASSFILE_H
#define _GLR_CLASSFILE_H

#include "../vm.h"

// class header -> class type (bottom 2 bits)
#define GLR_CLASS_ENUM   0
#define GLR_CLASS_TRAIT  1
#define GLR_CLASS_STRUCT 2
#define GLR_CLASS_MODULE 3

// class header -> class access (top 2 bits after class type)
#define GLR_ACCESS_PUB    (1 << 0)
#define GLR_ACCESS_CONST  (1 << 1)
#define GLR_ACCESS_STATIC (1 << 2)

// class header -> method -> func_type
#define GLR_FUNC_CALL  0  // interpreter method call
#define GLR_FUNC_TCALL 1  // interpreter tail call
#define GLR_FUNC_VCALL 2  // interpreter virtual call
#define GLR_FUNC_JCALL 3  // interpreter jit call

typedef struct {
    uint8_t type;
    union {
        double f64;
        int64_t i64;
        uint64_t u64;
        struct {
            char* text;
            size_t size;
        } string;
    } data;
} glr_const_t;

typedef struct glr_type_t {
    uint16_t type;
    struct glr_type_t* next;
} glr_type_t;

typedef struct {
    size_t size;
    glr_const_t* pool;
} glr_const_pool_t;

typedef struct glr_field_t {
    uint8_t access;
    uint16_t name;
    union {
        uint16_t type;
        glr_type_t* types;
    } type;
    struct glr_field_t* next;
} glr_field_t;

typedef struct glr_method_t {
    uint8_t access;
    uint8_t call_type;
    uint16_t name;
    glr_type_t* args;
    uint8_t* code;
    struct glr_method_t* next;
} glr_method_t;

typedef struct {
    uint8_t header;
    uint16_t name;
    glr_field_t* fields;
    glr_method_t* methods;
    glr_const_pool_t const_pool;
} glr_classfile_t;

typedef struct {
    int x;
} glr_class_loader_t;

typedef enum {
    GLR_CLASS_ERR_MAGIC      = 0,
    GLR_CLASS_ERR_VERSION    = 1,
    GLR_CLASS_ERR_BAD_CONST  = 2,
    GLR_CLASS_ERR_CONST_LEN  = 3,
    GLR_CLASS_ERR_FIELD_LEN  = 4,
    GLR_CLASS_ERR_METHOD_LEN = 5,
    GLR_CLASS_ERROR = 0xff
} glr_class_result;

glr_class_result glr_class_load(glr_vm_t* vm, uint8_t* bytes, size_t size);

#endif // _GLR_CLASSFILE_H
#ifndef _GLR_CLASSFILE_H
#define _GLR_CLASSFILE_H

#include "const_pool.h"
#include "../../os/info.h"

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

typedef struct glr_type_t {
    uint16_t type;
    struct glr_type_t* next;
} glr_type_t;

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
    glr_string_t* call_type;
    glr_string_t* name;
    glr_type_t* args;
    uint8_t* code;
    struct glr_method_t* next;
} glr_method_t;

typedef struct {
    uint8_t header;
    size_t next_class;
    glr_string_t* name;
    glr_field_t* fields;
    glr_method_t* methods;
    glr_const_pool_t const_pool;
} glr_classfile_t;

typedef struct {
    size_t size;
    size_t capacity;
    glr_sysinfo_t* info;
    glr_classfile_t** classes;
} glr_classloader_t;

typedef enum {
    GLR_CLASS_ERR_MAGIC      = 0,
    GLR_CLASS_ERR_VERSION    = 1,
    GLR_CLASS_ERR_ACCESS     = 2,
    GLR_CLASS_ERR_BAD_CONST  = 3,
    GLR_CLASS_ERR_CONST_LEN  = 4,
    GLR_CLASS_ERR_FIELD_LEN  = 5,
    GLR_CLASS_ERR_METHOD_LEN = 6,
    GLR_CLASS_ERROR = 0xff
} glr_class_result;

void glr_class_init(glr_classloader_t* loader, glr_sysinfo_t* info);

uintptr_t glr_class_load(glr_classloader_t* loader, uint8_t* bytes, size_t size);

void glr_class_insert(glr_classloader_t* loader, glr_classfile_t* class_file);

glr_classfile_t* glr_class_find(glr_classloader_t* loader, glr_string_t* class_name);

#endif // _GLR_CLASSFILE_H
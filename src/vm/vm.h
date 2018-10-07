#ifndef _GLR_VM_H
#define _GLR_VM_H

#include "../os/sys.h"
#include "../os/info.h"

typedef struct {
    uint8_t* top;
    uint8_t* heap;
    size_t capacity;
} glr_heap_t;

typedef struct {
    glr_heap_t class_heap;
} glr_vm_t;

void glr_vm_init(glr_vm_t* vm);

#endif // _GLR_VM_H
#ifndef _GLR_VM_H
#define _GLR_VM_H

#include "../os/sys.h"
#include "../os/info.h"
#include "class_file/class_file.h"

typedef struct {
    glr_classloader_t class_loader;
} glr_vm_t;

void glr_vm_init(glr_vm_t* vm);

#endif // _GLR_VM_H
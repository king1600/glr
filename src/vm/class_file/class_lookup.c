#include "class_file.h"
#include "../../os/page.h"
#include "../../os/hash.h"
#include "../../os/offsets.h"

#define GLR_CLASS_ALLOC(var, offset, size) \
    GLR_PAGE_ALLOC(var, GLR_OFFSET_CLASSFILE, offset, size "class files")

void glr_class_init(glr_classloader_t* loader, glr_sysinfo_t* info) {
    loader->size = 0;
    loader->info = info;
    loader->capacity = 0;
    loader->classes = NULL;
}

void glr_class_insert(glr_classloader_t* loader, glr_classfile_t* class_file) {
    if (GLR_UNLIKELY(loader->capacity == 0)) {
        GLR_CLASS_ALLOC(loader->classes, 0, loader->info->page_size);
    }

    if (((++loader->size) * sizeof(glr_classfile_t*)) > loader->capacity) {
        GLR_CLASS_ALLOC(void* _, loader->capacity, loader->info->page_size);
    }

    const size_t mask = loader->capacity - 1;
    glr_string_t* class_name = class_file->name;
    size_t index = ((size_t) glr_hash_string(name->text, name->size)) & mask;

    class_file->next_class = 0;
    for (size_t probes = 0; probes < loader->capacity; probes++) {
        glr_classfile_t** other = &loader->classes[index];

        if (*other == NULL) {
            *other = class_file;
            return;

        } else {
            if ((*other)->next_class < class_file->next_class) {
                glr_classfile_t* temp = *other;
                *other = class_file;
                class_file = temp;
            }
            class_file->next_class++;
        }
    }
}

glr_classfile_t* glr_class_find(glr_classloader_t* loader, glr_string_t* class_name) {
    return NULL;
}

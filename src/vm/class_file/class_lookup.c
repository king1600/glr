#include "string.h"
#include "class_file.h"
#include "../../os/page.h"
#include "../../os/hash.h"
#include "../../os/offsets.h"

#define GLR_CLASS_ALLOC(var, offset, size) \
    GLR_PAGE_ALLOC(var, GLR_OFFSET_CLASSFILE, offset, size, "class files")

void glr_class_init(glr_classloader_t* loader, glr_sysinfo_t* info) {
    loader->size = 0;
    loader->info = info;
    loader->capacity = 0;
    loader->classes = NULL;
}

void glr_class_free(glr_classloader_t* loader) {
    if (GLR_LIKELY(loader->classes != NULL)) {
        loader->size = 0;
        const size_t page = loader->info->page_size;

        // free all the pages allocated
        while (loader->size * page < loader->capacity)
            glr_page_free(loader->classes + (loader->size++ * page), page);
    }
}

glr_classfile_t* glr_class_find(glr_classloader_t* loader, glr_string_t* class_name) {
    // odd path where theres no classes inserted
    if (GLR_UNLIKELY(loader->capacity == 0))
        return NULL;

    // calculate the hashed index to start looking at
    const char* text = class_name->text;
    const size_t mask = loader->capacity - 1;
    const size_t start = ((size_t) glr_hash_string(class_name->text, class_name->size)) & mask;
    
    // try and find the classfile by probing for its name
    // returning null when it has completely looped around
    size_t index = start;
    glr_classfile_t* class_file = loader->classes[index];
    do {
        if (class_file != NULL) {
            const char* name = class_file->name->text;
            const size_t size = GLR_MIN(class_file->name->size, class_name->size);
            if (strncmp(name, text, size) == 0)
                return class_file;
        }
        index = (index + 1) & mask;
    } while (index != start);

    // couldnt find it
    return NULL;
}

void glr_class_insert(glr_classloader_t* loader, glr_classfile_t* class_file) {
    const size_t page_size = loader->info->page_size;

    // make sure theres class slot allocated
    if (GLR_UNLIKELY(loader->capacity == 0)) {
        GLR_CLASS_ALLOC(loader->classes, 0, page_size);
        memset(loader->classes, 0, page_size);
    }

    // ensure that theres enough slots to be inserted next
    if (GLR_UNLIKELY(((++loader->size) * sizeof(glr_classfile_t*)) > loader->capacity)) {
        void* page;
        GLR_CLASS_ALLOC(page, loader->capacity, page_size);
        memset(page, 0, page_size);
    }

    // calculate the hashed index to start looking at
    const size_t mask = loader->capacity - 1;
    glr_string_t* class_name = class_file->name;
    size_t index = ((size_t) glr_hash_string(class_name->text, class_name->size)) & mask;

    // probe through the hashmap at least $capacity times in case of collisions
    class_file->next_class = 0;
    for (size_t probes = 0; probes < loader->capacity; probes++) {
        glr_classfile_t** other = &loader->classes[index];

        // empty slot found, place classfile here
        if (*other == NULL) {
            *other = class_file;
            return;

        // collision occured, swap classfile with this index
        // and try to move the one previously at this index elsewhere
        } else if ((*other)->next_class < class_file->next_class) {
            glr_classfile_t* temp = *other;
            *other = class_file;
            class_file = temp;
        }

        // try the next probed index
        class_file->next_class++;
        index = (index + 1) & mask;
    }
}

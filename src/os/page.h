#ifndef _GLR_PAGE_H
#define _GLR_PAGE_H

#include "handle.h"

#define GLR_PAGE_EXEC   (1 << 0)
#define GLR_PAGE_READ   (1 << 1)
#define GLR_PAGE_WRITE  (1 << 2)
#define GLR_PAGE_HUGE   (1 << 3)
#define GLR_PAGE_COMMIT (1 << 4)

// Virtual Memory Functions

#define GLR_PAGE_OFFSET(base, offset) \
    ((void*) ((base) + (offset)))

#define GLR_PAGE_ALLOC(var, base, offset, size, type)   \
    var = glr_page_alloc(GLR_PAGE_OFFSET(base, offset), \
                size,  GLR_PAGE_READ | GLR_PAGE_WRITE); \
    GLR_ASSERT((var) == GLR_PAGE_OFFSET(base, offset),  \
        "Failed to map data for " type)


bool glr_page_free(void* addr, size_t size);

bool glr_page_release(void* addr, size_t size);

void* glr_page_alloc(void* addr, size_t size, int flags);

// Physical Memory Functions

bool glr_mem_free(GLR_FD fd, size_t size);

GLR_FD glr_mem_alloc(size_t size, int flags);

bool glr_mem_unmap(void* addr, size_t size);

void* glr_mem_map(GLR_FD fd, void* addr, size_t size, int flags);


#endif // _GLR_PAGE_H
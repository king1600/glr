#include "page.h"

typedef struct {
    uint32_t memory;
    uint32_t protect;
} page_flag_values;

#define GLR_PAGE_PROTECT (GLR_PAGE_EXEC | GLR_PAGE_READ | GLR_PAGE_WRITE)

#ifdef GLR_WINDOWS
    // GLR_PAGE_* flags To System Flags Conversion
        
    inline DWORD page_flags_memory(int flags, DWORD huge, DWORD commit, DWORD reserve) {
        DWORD page_memory = 0;

        if (flags & GLR_PAGE_HUGE)
            page_memory |= huge;
        page_memory |= flags & GLR_PAGE_COMMIT ? commit : reserve;

        return page_memory;
    }

    inline DWORD page_flags_protect(int flags) {
        DWORD page_protect = 0;

        switch (flags & GLR_PAGE_PROTECT) {
            default:
                page_protect = PAGE_NOACCESS;
                break;
            case GLR_PAGE_EXEC: 
                page_protect = PAGE_EXECUTE;
                break;
            case GLR_PAGE_READ:
                page_protect = PAGE_READONLY;
                break;
            case GLR_PAGE_EXEC | GLR_PAGE_READ:
                page_protect = PAGE_EXECUTE_READ;
                break;
            case GLR_PAGE_WRITE:
            case GLR_PAGE_WRITE | GLR_PAGE_READ:
                page_protect = PAGE_READWRITE;
                break;
            case GLR_PAGE_PROTECT:
            case GLR_PAGE_EXEC | GLR_PAGE_WRITE:
                page_protect = PAGE_EXECUTE_READWRITE;
                break;
        }

        return page_protect;
    }

    // Virtual Memory Functions
    bool glr_page_free(void* addr, size_t size) {
        return VirtualFree(addr, size, MEM_RELEASE);
    }

    bool glr_page_release(void* addr, size_t size) {
        return DiscardVirtualMemory(addr, size) == ERROR_SUCCESS;
    }
    
    void* glr_page_alloc(void* addr, size_t size, int flags) {
        DWORD memory = page_flags_memory(flags, MEM_LARGE_PAGES, MEM_COMMIT, MEM_RESERVE);
        DWORD protect = page_flags_protect(flags);
        return VirtualAlloc(addr, size, memory, protect);
    }

    // Physical Memory Functions
    bool glr_mem_free(GLR_FD fd, size_t size) {
        return glr_fd_close(fd);
    }

    GLR_FD glr_mem_alloc(size_t size, int flags) {
        DWORD protect = page_flags_protect(flags)
            | page_flags_memory(flags, SEC_LARGE_PAGES, SEC_COMMIT, SEC_RESERVE);
        return CreateFileMapping(GLR_BAD_FD, NULL, protect, size >> 32, (DWORD) size, NULL);
    }

    bool glr_mem_unmap(void* addr, size_t size) {
        return UnmapViewOfFile(addr);
    }

    void* glr_mem_map(GLR_FD fd, void* addr, size_t size, int flags) {
        return MapViewOfFileEx(fd, FILE_MAP_ALL_ACCESS, 0, 0, size, addr);
    }

#else
    #include <fcntl.h>
    #include <stdio.h>
    #include <sys/mman.h>
    #include "atomic.h"
    
    static GLR_ATOMIC(uint64_t) physical_backing_id = 0;

    // Virtual Memory Functions

    bool glr_page_free(void* addr, size_t size) {
        return munmap(addr, size) == 0;
    }

    bool glr_page_release(void* addr, size_t size) {
        return madvise(addr, size, MADV_DONTNEED) == 0;
    }
    
    void* glr_page_alloc(void* addr, size_t size, int flags) {
        return glr_mem_map(GLR_BAD_FD, addr, size, flags);
    }

    // Physical Memory Functions

    bool glr_mem_free(GLR_FD fd, size_t size) {
        return glr_fd_close(fd);
    }

    GLR_FD glr_mem_alloc(size_t size, int flags) {
        GLR_FD fd;
        char path[128] = { 0 };

        uint64_t id = glr_atomic_add(&physical_backing_id, 1, GLR_ATOMIC_RELAXED);
        snprintf(path, sizeof(path), "/glr(%d,%lu)", getpid(), id);
        if ((fd = shm_open(path, O_RDWR | O_CREAT | O_EXCL, 0600)) == GLR_BAD_FD)
            return GLR_BAD_FD;

        shm_unlink(path);
        ftruncate(fd, size);
        return fd;
    }

    bool glr_mem_unmap(void* addr, size_t size) {
        return munmap(addr, size);
    }

    void* glr_mem_map(GLR_FD fd, void* addr, size_t size, int flags) {
        int page_protect = 0;
        int page_memory = MAP_PRIVATE | MAP_ANONYMOUS;
        
        if (flags & GLR_PAGE_EXEC)
            page_protect |= PROT_EXEC;
        if (flags & GLR_PAGE_READ)
            page_protect |= PROT_READ;
        if (flags & GLR_PAGE_WRITE)
            page_protect |= PROT_WRITE;

        if (addr != NULL)
            page_memory |= MAP_FIXED;
        if (flags & GLR_PAGE_HUGE)
            page_memory |= MAP_HUGETLB;
        if ((flags & GLR_PAGE_COMMIT) && (page_memory & MAP_PRIVATE))
            page_memory |= MAP_POPULATE;
        else
            page_memory |= MAP_NORESERVE;

        addr = mmap(addr, size, page_protect, page_memory, fd, 0);
        return addr == MAP_FAILED ? NULL : addr;
    }

#endif

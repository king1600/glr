pub struct Page {
    ptr: *mut u8,
    size: usize,
    file: bool,
}

#[cfg(not(windows))]
mod memory {
    use super::super::{*, types::*};

    use libc::{
        mmap, munmap,
        shm_open, shm_unlink,
        PROT_EXEC, PROT_READ, PROT_WRITE,
        O_RDWR, O_EXCL, O_CREAT, ftruncate,
        MAP_FAILED, MAP_PRIVATE, MAP_ANONYMOUS, MAP_FIXED, MAP_HUGETLB, MAP_POPULATE,
    };

    pub fn page_from_file(file: File, addr: usize, size: usize) -> Option<Page> {
        let prot = PROT_READ | PROT_WRITE;
        let mut flags = MAP_PRIVATE;
        if addr > 0 { flags |= MAP_FIXED }

        match unsafe { mmap(addr as *mut c_void, size, prot, flags, file.fd, 0) } {
            MAP_FAILED => None,
            ptr => Some(Page {
                size: size,
                file: true,
                ptr: ptr as *mut u8
            })
        }
    }

    pub fn alloc_file(path: &str, size: usize, _large_pages: bool) -> Option<File> {
        unsafe {
            let fd = shm_open(path.c_str(), O_RDWR | O_CREAT | O_EXCL, 384);
            if fd > 0 {
                shm_unlink(path.c_str());
                ftruncate(fd, size as i64);
                Some(File::from(fd))
            } else {
                None
            }
        }
    }

    #[inline]
    pub unsafe fn free_page(page: &Page) {
        munmap(page.addr as *mut c_void, page.size);
    }

    pub fn alloc_page(
        addr: usize, size: usize,
        pretouch: bool, executable: bool, large_pages: bool
    ) -> Option<Page> {
        let mut flags = PROT_READ | PROT_WRITE;    
        let mut prot = MAP_PRIVATE | MAP_ANONYMOUS;
        
        if addr > 0 { prot |= MAP_FIXED }
        if executable { flags |= PROT_EXEC }
        if pretouch { prot |= MAP_POPULATE }
        if large_pages { prot |= MAP_HUGETLB }

        match unsafe { mmap(addr as *mut c_void, size, prot, flags, -1, 0) } {
            MAP_FAILED => None,
            ptr => Some(Page {
                size: size,
                file: false,
                ptr: ptr as *mut u8
            })
        }
    }
}

#[cfg(windows)]
mod memory {
    use super::super::{*, types::*};

    use winapi::um::{
        winnt::{
            SEC_COMMIT, SEC_LARGE_PAGES,
            PAGE_READWRITE, PAGE_EXECUTE_READWRITE,
            MEM_COMMIT, MEM_RESERVE, MEM_LARGE_PAGES, MEM_RELEASE,
        },
        memoryapi::{
            FILE_MAP_ALL_ACCESS,
            VirtualAlloc, VirtualFree,
            CreateFileMappingW, MapViewOfFileEx, UnmapViewOfFile,
        },
    };

    // addr must be a multiple of system granularity
    #[inline]
    pub fn page_from_file(file: File, addr: usize, size: usize) -> Option<Page> {
        match unsafe { MapViewOfFileEx(file.fd, FILE_MAP_ALL_ACCESS, 0, 0, size, addr as *mut c_void) } {
            NULL => None,
            ptr => Some(Page {
                size: size,
                file: true,
                ptr: ptr as *mut u8,
            })
        }
    }

    pub fn alloc_file(_path: &str, size: usize, large_pages: bool) -> Option<File> {
        let handle = INVALID_HANDLE;
        let mut flags = PAGE_READWRITE | SEC_COMMIT;
        if large_pages { flags |= SEC_LARGE_PAGES }

        let size_low = size as DWORD;
        let size_high = (size >> 32) as DWORD;
        let name = NULL as *mut _;
        let attributes = NULL as *mut _;

        match unsafe { CreateFileMappingW(handle, attributes, flags, size_high, size_low, name) } {
            NULL => None,
            fd => Some(File::from(fd)),
        }
    }

    #[inline]
    pub unsafe fn free_page(page: &Page) {
        if page.file {
            UnmapViewOfFile(page.ptr as LPVOID);
        } else {
            VirtualFree(page.ptr as LPVOID, page.size, MEM_RELEASE);
        }
    }

    pub fn alloc_page(
        addr: usize, size: usize,
        pretouch: bool, executable: bool, large_pages: bool
    ) -> Option<Page> {
        let mut prot = MEM_RESERVE;
        if pretouch { prot |= MEM_COMMIT }
        if large_pages { prot |= MEM_LARGE_PAGES }

        let flags = if executable {
            PAGE_EXECUTE_READWRITE
        } else {
            PAGE_READWRITE
        };

        match unsafe { VirtualAlloc(addr as LPVOID, size, prot, flags) } {
            NULL => None,
            ptr => Some(Page {
                size: size,
                file: false,
                ptr: ptr as *mut u8
            })
        }
    }
}
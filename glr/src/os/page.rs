pub struct Page {
    ptr: *mut u8,
    size: usize,
}

#[cfg(not(windows))]
mod memory {
    use super::{Page, super::{CString, file::File}};

    use libc::{mmap, c_void};
    use libc::{O_RDWR, O_CREAT, O_EXCL};
    use libc::{shm_open, shm_unlink, ftruncate};
    use libc::{PROT_EXEC, PROT_READ, PROT_WRITE};
    use libc::{MAP_FAILED, MAP_PRIVATE, MAP_ANONYMOUS, MAP_FIXED, MAP_HUGETLB, MAP_POPULATE};

    pub fn from_file(file: File, addr: usize, size: usize) -> Option<Page> {
        let prot = PROT_READ | PROT_WRITE;
        let mut flags = MAP_PRIVATE;
        if addr > 0 { flags |= MAP_FIXED }

        match unsafe { mmap(addr as *mut c_void, size, prot, flags, file.fd, 0) } {
            MAP_FAILED => None,
            ptr => Some(Page {
                size: size,
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
                Some((fd as i32).into())
            } else {
                None
            }
        }
    }

    pub fn alloc(
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
                ptr: ptr as *mut u8
            })
        }
    }
}

#[cfg(windows)]
mod memory {
    use super::super::CString;
    use libc::NULL;
    use kernel32::{VirtualAlloc, LPVOID};
    use kernel32::{PAGE_EXECUTE_READWRITE, PAGE_READWRITE};
    use kernel32::{MEM_COMMIT, MEM_RESERVE, MEM_LARGE_PAGES};
    use kernel32::{SEC_COMMIT, SEC_LARGE_PAGES, INVALID_HANDLE_VALUE};
    use kernel32::{CreateFileMapping, MapViewOfFileEx, FILE_MAP_ALL_ACCESS};

    // addr must be a multiple of system granularity
    #[inline]
    pub fn from_file(fd: i32, addr: usize, size: usize) -> Option<Page> {
        match unsafe { MapViewOfFileEx(fd, FILE_MAP_ALL_ACCESS, 0, 0, size, addr) } {
            NULL => None,
            ptr => Some(Page {
                size: size,
                ptr: ptr as *mut u8,
            })
        }
    }

    pub fn alloc_file(path: &str, size: usize, large_pages: bool) -> Option<File> {
        let handle = INVALID_HANDLE_VALUE;
        let mut flags = PAGE_READWRITE | SEC_COMMIT;
        if large_pages { flags |= SEC_LARGE_PAGES }

        match unsafe { CreateFileMapping(handle, NULL, flags, 0, size, NULL) } {
            NULL => None,
            fd => Some((fd as i32).into()),
        }
    }

    pub unsafe fn alloc(
        addr: usize, size: usize, _flags: i32,
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
                ptr: ptr as *mut u8
            })
        }
    }
}
use super::{*, types::*};

pub struct Page {
    pub(super) size: usize,
    pub(super) ptr: *mut u8,
    #[cfg(windows)] is_file: bool,
}

pub const PAGE_EXEC:      i32 = 1 << 0;
pub const PAGE_HUGE:      i32 = 1 << 1;
pub const PAGE_PRETOUCH:  i32 = 1 << 2;
pub const PAGE_READWRITE: i32 = 1 << 3;

impl Drop for Page {
    fn drop(&mut self) {
        if self.size > 0 {
            unsafe { self.free() }
        }
    }
}

#[cfg(unix)]
use libc::{
    mmap, munmap,
    shm_open, shm_unlink,
    PROT_EXEC, PROT_READ, PROT_WRITE,
    O_RDWR, O_EXCL, O_CREAT, ftruncate,
    MAP_FAILED, MAP_PRIVATE, MAP_SHARED, MAP_ANONYMOUS, MAP_FIXED, MAP_HUGETLB, MAP_POPULATE,
};

#[cfg(unix)]
impl Page {
    unsafe fn free(&mut self) {
        munmap(self.ptr as *mut c_void, self.size);
    }

    #[inline]
    pub fn alloc(addr: usize, size: usize, flags: i32) -> Option<Page> {
        unsafe { Self::alloc_inner(addr, size, flags, -1, MAP_PRIVATE | MAP_ANONYMOUS) }
    }

    #[inline]
    pub fn from_file(file: &File, addr: usize, size: usize, flags: i32) -> Option<Page> {
        unsafe { Self::alloc_inner(addr, size, flags, file.fd, MAP_SHARED) }
    }

    pub fn alloc_file(path: &str, size: usize, _flags: i32) -> Option<File> {
        unsafe {
            let fd = shm_open(path.c_str(), O_EXCL | O_RDWR | O_CREAT, 0600);
            if fd > 0 {
                shm_unlink(path.c_str());
                ftruncate(fd, size as i64);
                Some(File::from(fd))
            } else {
                None
            }
        }
    }

    unsafe fn alloc_inner(addr: usize, size: usize, flags: i32, fd: HANDLE, mut mapping: c_int) -> Option<Page> {
        let mut protect = 0;
        if flags & PAGE_EXEC != 0 { protect |= PROT_EXEC }
        if flags & PAGE_READWRITE != 0 { protect |= PROT_READ | PROT_WRITE }

        if addr > 0 { mapping |= MAP_FIXED }
        if flags & PAGE_HUGE != 0 { mapping |= MAP_HUGETLB }
        if flags & PAGE_PRETOUCH != 0 { mapping |= MAP_POPULATE }

        match mmap(addr as *mut c_void, size, protect, mapping, fd, 0) {
            MAP_FAILED => None,
            ptr => Some(Page {
                size: size,
                ptr: ptr as *mut u8
            })
        }
    }
}

#[cfg(windows)]
use winapi::um::{
    winnt::{
        SEC_COMMIT, SEC_LARGE_PAGES,
        MEM_RESERVE, MEM_RELEASE, MEM_LARGE_PAGES,
        PAGE_READWRITE as READWRITE, PAGE_EXECUTE_READWRITE as EXEC_READWRITE,
    },
    memoryapi::{
        FILE_MAP_ALL_ACCESS,
        VirtualAlloc, VirtualFree,
        CreateFileMappingW, MapViewOfFileEx, UnmapViewOfFile,
    },
};

#[cfg(windows)]
impl Page {
    unsafe fn free(&mut self) {
        if self.is_file {
            UnmapViewOfFile(self.ptr as LPVOID);
        } else {
            VirtualFree(self.ptr as LPVOID, self.size, MEM_RELEASE);
        }
    }

    pub fn from_file(file: &File, addr: usize, size: usize, _flags: i32) -> Option<Page> {
        match unsafe { MapViewOfFileEx(file.fd, FILE_MAP_ALL_ACCESS, 0, 0, size, addr as *mut c_void) } {
            NULL => None,
            ptr => Some(Page {
                size: size,
                is_file: true,
                ptr: ptr as *mut u8,
            })
        }
    }

    pub fn alloc(addr: usize, size: usize, flags: i32) -> Option<Page> {
        let mut protect = 0;
        let mut mapping = MEM_RESERVE;

        if flags & PAGE_READWRITE != 0 { protect |= READWRITE }
        if flags & PAGE_EXEC != 0 { protect |= EXEC_READWRITE }
        if flags & PAGE_HUGE != 0 { mapping |= MEM_LARGE_PAGES }

        match unsafe { VirtualAlloc(addr as LPVOID, size, protect, mapping) } {
            NULL => None,
            ptr => Some(Page {
                size: size,
                is_file: false,
                ptr: ptr as *mut u8,
            })
        }
    }

    pub fn alloc_file(_path: &str, size: usize, flags: i32) -> Option<File> {
        let name = NULL as *mut _;
        let attr = NULL as *mut _;

        let handle = INVALID_HANDLE;
        let size_low = size as DWORD;
        let size_high = (size >> 32) as DWORD;
        
        let mut mapping = SEC_COMMIT | READWRITE;
        if flags & PAGE_HUGE != 0 { mapping |= SEC_LARGE_PAGES }

        match unsafe { CreateFileMappingW(handle, attr, mapping, size_high, size_low, name) } {
            NULL => None,
            fd => Some(File::from(fd))
        }
    }
}
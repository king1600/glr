use super::*;

pub const PAGE_EXEC:   i32 = 1 << 0;
pub const PAGE_READ:   i32 = 1 << 1;
pub const PAGE_WRITE:  i32 = 1 << 2;
pub const PAGE_HUGE:   i32 = 1 << 3;
pub const PAGE_COMMIT: i32 = 1 << 4;

#[cfg(unix)] type PageHandle = i32;
#[cfg(windows)] type PageHandle = HANDLE;

pub struct PhysicalPage {
    size: usize,
    handle: PageHandle,
}

pub struct Page {
    pub addr: usize,
    pub size: usize,
    #[cfg(windows)] mapped: bool,
}

impl Drop for PhysicalPage {
    fn drop(&mut self) {
        unsafe { page_impl::physical_drop(&self); }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe { page_impl::virtual_drop(&self); }
    }
}

impl PhysicalPage {
    #[inline]
    pub unsafe fn from(handle: PageHandle, size: usize) -> PhysicalPage {
        PhysicalPage { handle, size }
    }

    #[inline]
    pub fn new(size: usize, flags: i32) -> Option<PhysicalPage> {
        unsafe { page_impl::physical_alloc(size, flags) }
    }

    #[inline]
    pub fn map(&self, addr: usize, size: usize, flags: i32) -> Option<Page> {
        unsafe { page_impl::physical_map(&self, addr, size, flags) }
    }
}

impl Page {
    #[cfg(unix)]
    pub unsafe fn from(addr: usize, size: usize) -> Page {
        Page { addr, size }
    }

    #[cfg(windows)]
    pub unsafe fn from(addr: usize, size: usize) -> Page {
        Page { addr, size, mapped: false }
    }

    #[inline]
    pub fn new(addr: usize, size: usize, flags: i32) -> Option<Page> {
        unsafe { page_impl::virtual_alloc(addr, size, flags) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn release(&self) {
        unsafe { page_impl::virtual_release(&self); }
    }

    #[inline]
    pub fn ptr<T>(&self, offset: usize) -> Option<*mut T> {
        if offset >= self.size {
            None
        } else {
            Some((self.addr + offset) as *mut T)
        }
    }
}

#[cfg(unix)]
mod page_impl {
    use super::{*, super::sync::{AtomicUsize, Ordering}};

    static mut PHYSICAL_ID: AtomicUsize = AtomicUsize::new(0);

    pub unsafe fn physical_drop(page: &PhysicalPage) {
        close(page.handle);
    }

    pub unsafe fn physical_map(page: &PhysicalPage, addr: usize, size: usize, flags: i32) -> Option<Page> {
        inner_map(addr, size, flags, page.handle, MAP_SHARED)
    }

    pub unsafe fn physical_alloc(size: usize, _flags: i32) -> Option<PhysicalPage> {
        let mut path = [0; 128];
        let phys_id = PHYSICAL_ID.fetch_add(1, Ordering::Relaxed);
        snprintf(path.as_mut_ptr(), path.len(), "/glr(%d,%lu)\0".c_str(), getpid(), phys_id);

        let handle = shm_open(path.as_ptr(), O_RDWR | O_CREAT | O_EXCL, 0o600);
        if handle != -1 {
            shm_unlink(path.as_ptr());
            ftruncate(handle, size as i64);
            Some(PhysicalPage { handle, size })
        } else {
            return None
        }
    }

    pub unsafe fn virtual_drop(page: &Page) {
        munmap(page.addr as *mut _, page.size);
    }

    pub unsafe fn virtual_release(page: &Page) {
        madvise(page.addr as *mut _, page.size, MADV_DONTNEED);
    }

    pub unsafe fn virtual_alloc(addr: usize, size: usize, flags: i32) -> Option<Page> {
        inner_map(addr, size, flags, -1, MAP_PRIVATE | MAP_ANONYMOUS)
    }

    unsafe fn inner_map(addr: usize, size: usize, flags: i32, fd: i32, mut memory: i32) -> Option<Page> {
        let mut protect = 0;

        if addr > 0 { memory |= MAP_FIXED }
        if flags & PAGE_EXEC != 0 { protect |= PROT_EXEC }
        if flags & PAGE_READ != 0 { protect |= PROT_READ }
        if flags & PAGE_WRITE != 0 { protect |= PROT_WRITE }
        if flags & PAGE_HUGE != 0 { memory |= MAP_HUGETLB }

        memory |= if (flags & PAGE_COMMIT != 0) && (memory & MAP_PRIVATE != 0) {
            MAP_POPULATE
        } else {
            MAP_NORESERVE
        };

        match mmap(addr as *mut _, size, protect, memory, fd, 0) {
            MAP_FAILED => None,
            addr => Some(Page {
                size: size,
                addr: addr as usize,
            })
        }
    }
}

#[cfg(windows)]
mod page_impl {
    use super::*;

    static VIRTUAL_FLAGS: [DWORD; 3] = [MEM_LARGE_PAGES, MEM_COMMIT, MEM_RESERVE];
    static PHYSICAL_FLAGS: [DWORD; 3] = [SEC_LARGE_PAGES, SEC_COMMIT, SEC_RESERVE];

    fn memory_flags(flags: i32, using: &[DWORD]) -> DWORD {
        const PAGE_HUGE_COMMIT: i32 = (PAGE_HUGE | PAGE_COMMIT);
        match flags & PAGE_HUGE_COMMIT {
            PAGE_COMMIT      => using[1],
            PAGE_HUGE        => using[0] | using[2],
            PAGE_HUGE_COMMIT => using[0] | using[1],
            _ => 0,
        }
    }

    fn protect_flags(flags: i32) -> DWORD {
        const PAGE_MASK: i32 = PAGE_EXEC | PAGE_READ | PAGE_WRITE;
        match flags & PAGE_MASK {
            PAGE_EXEC => PAGE_EXECUTE,
            PAGE_READ => PAGE_READONLY,
            f if f == (PAGE_READ | PAGE_EXEC) => PAGE_EXECUTE_READ,
            f if f == (PAGE_READ | PAGE_WRITE) | PAGE_WRITE => PAGE_READWRITE,
            f if f == (PAGE_EXEC | PAGE_WRITE) | PAGE_MASK => PAGE_EXECUTE_READWRITE,
            _ => PAGE_NOACCESS
        }
    }

    pub unsafe fn physical_drop(page: &PhysicalPage) {
        CloseHandle(page.handle);
    }

    pub unsafe fn physical_map(page: &PhysicalPage, addr: usize, size: usize, _flags: i32) -> Option<Page> {
        match MapViewOfFileEx(page.handle, FILE_MAP_ALL_ACCESS, 0, 0, size, addr as *mut _) {
            NULL => None,
            addr => Some(Page {
                size: size,
                mapped: true,
                addr: addr as usize
            })
        }
    }

    pub unsafe fn physical_alloc(size: usize, flags: i32) -> Option<PhysicalPage> {
        let (size_high, size_low) = ((size >> 32) as DWORD, size as DWORD);
        let flags = protect_flags(flags) | memory_flags(flags, &PHYSICAL_FLAGS);

        match CreateFileMappingW(INVALID_HANDLE_VALUE, null_mut(), flags, size_high, size_low, null_mut()) {
            NULL => None,
            handle => Some(PhysicalPage { handle, size })
        }     
    }

    pub unsafe fn virtual_drop(page: &Page) {
        if page.mapped {
            UnmapViewOfFile(page.addr as *mut _);
        } else {
            VirtualFree(page.addr as *mut _, 0, MEM_RELEASE);
        }
    }

    pub unsafe fn virtual_release(page: &Page) {
        DiscardVirtualMemory(page.addr as *mut _, page.size);
    }

    pub unsafe fn virtual_alloc(addr: usize, size: usize, flags: i32) -> Option<Page> {
        let protect = protect_flags(flags);
        let memory = memory_flags(flags, &VIRTUAL_FLAGS);
        match VirtualAlloc(addr as *mut _, size, memory, protect) {
            NULL => None,
            addr => Some(Page {
                size: size,
                mapped: false,
                addr: addr as usize,
            })
        }
    }
}
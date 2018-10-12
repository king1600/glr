use super::*;

pub const PAGE_EXEC:   i32 = 1 << 0;
pub const PAGE_READ:   i32 = 1 << 1;
pub const PAGE_WRITE:  i32 = 1 << 2;
pub const PAGE_HUGE:   i32 = 1 << 3;
pub const PAGE_COMMIT: i32 = 1 << 4;

pub struct PhysicalPage {
    #[cfg(unix)] handle: i32,
    #[cfg(windows)] handle: HANDLE,
}

pub struct Page {
    addr: usize,
    size: usize,
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
    pub fn new(size: usize, flags: i32) -> Option<PhysicalPage> {
        unsafe { page_impl::physical_alloc(size, flags) }
    }

    #[inline]
    pub fn map(&self, addr: usize, size: usize, flags: i32) -> Option<Page> {
        unsafe { page_impl::physical_map(&self, addr, size, flags) }
    }
}

impl Page {
    #[inline]
    pub fn new(addr: usize, size: usize, flags: i32) -> Option<Page> {
        unsafe { page_impl::virtual_alloc(addr, size, flags) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn ptr<T>(&self, offset: usize) -> *mut T {
        (self.addr + offset) as *mut T
    }

    #[inline]
    pub fn release(&self) {
        unsafe { page_impl::virtual_release(&self); }
    }
}

#[cfg(unix)]
mod page_impl {
    use super::*;
    
    static mut physical_id: usize = 0;

    pub unsafe fn physical_drop(page: &PhysicalPage) {
        close(page.handle);
    }

    pub unsafe fn physical_map(page: &PhysicalPage, addr: usize, size: usize, flags: i32) -> Option<Page> {
        
    }

    pub unsafe fn physical_alloc(size: usize, flags: i32) -> Option<PhysicalPage> {
    }

    pub unsafe fn virtual_drop(page: &Page) {

    }

    pub unsafe fn virtual_release(page: &Page) {

    }

    pub unsafe fn virtual_alloc(addr: usize, size: usize, flags: i32) -> Option<Page> {

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
        let name = NULL as *mut _;
        let attributes = NULL as *mut _;
        let (size_high, size_low) = ((size >> 32) as DWORD, size as DWORD);
        let flags = protect_flags(flags) | memory_flags(flags, &PHYSICAL_FLAGS);

        match CreateFileMappingW(INVALID_HANDLE_VALUE, attributes, flags, size_high, size_low, name) {
            NULL => None,
            handle => Some(PhysicalPage { handle })
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
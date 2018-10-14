use super::*;

pub const STDOUT_BUFFER: usize = (1 << 14); // 16kb of addressable memory
pub const STDERR_BUFFER: usize = (1 << 15); // 32kb of addressable memory
pub const CLASS_MAPPING: usize = (1 << 25); // 32mb of addressable memory
pub const CLASS_MEMORY:  usize = (1 << 30); // 1gb of addressable memory

lazy_static! {
    static ref PAGE_SIZES: (usize, usize) = unsafe { get_page_sizes() };
}

#[cfg(windows)]
#[inline(always)]
unsafe fn get_page_sizes() -> (usize, usize) {
    let mut info: SYSTEM_INFO = core::mem::uninitialized();
    GetSystemInfo(&mut info);
    (info.dwPageSize as usize, GetLargePageMinimum() as usize)
}

#[cfg(unix)]
#[inline(always)]
unsafe fn get_page_sizes() -> (usize, usize) {
    let page_size = sysconf(_SC_PAGESIZE);
    let huge_page_size = 2 * 1024 * 1024; // TODO: Check the system for it
    (page_size as usize, huge_page_Size)
}

pub struct MemoryRange {
    top: usize,
    addr: usize,
    size: usize,
}

impl core::ops::Drop for MemoryRange {
    fn drop(&mut self) {
        unsafe {
            #[cfg(unix)] munmap(self.addr as *mut c_void, self.size);
            #[cfg(windows)] VirtualFree(self.addr as *mut c_void, 0, MEM_RELEASE);
        }
    }
}

impl MemoryRange {
    pub fn at(offset: usize) -> Option<MemoryRange> {
        let top = 0;
        let size = (offset << 1) - offset;
        (unsafe { Self::mmap(offset as *mut c_void, size) })
            .and_then(|addr| Some(Self { top, addr, size }))
    }

    #[inline]
    pub fn page_size() -> usize {
        PAGE_SIZES.0
    }

    #[inline]
    pub fn huge_page_size() -> usize {
        PAGE_SIZES.1
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn drain(&mut self) -> &[u8] {
        let mut buffer_end = 0;
        core::mem::swap(&mut self.top, &mut buffer_end);
        &self.as_bytes()[..buffer_end]
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.addr as *const _, self.size) }
    }

    pub fn alloc<T: Sized>(&mut self) -> Option<&mut T> {
        self.alloc_bytes(core::mem::size_of::<T>())
            .and_then(|bytes| Some(unsafe {
                &mut *(bytes as *mut _)
            }))
    }

    pub fn alloc_bytes(&mut self, bytes: usize) -> Option<*mut u8> {
        unsafe {
            if likely(self.top + bytes <= self.size) {
                let ptr = self.top;
                self.top += bytes;
                Some((self.addr + ptr) as *mut _)
            } else {
                None
            }
        }
    }

    #[cfg(unix)]
    unsafe fn mmap(addr: *mut c_void, size: usize) -> Option<usize> {
        let mut flags = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE | MAP_FIXED;
        if size >= Self::huge_page_size() {
            flags |= MAP_HUGETLB
        }
        match mmap(addr, size, PROT_READ | PROT_WRITE, flags, -1, 0) {
            MAP_FAILED => None,
            addr => Some(addr as usize),
        }
    }

    #[cfg(windows)]
    unsafe fn mmap(addr: *mut c_void, size: usize) -> Option<usize> {
        let mut flags = MEM_RESERVE;
        if size >= Self::huge_page_size() {
            flags |= MEM_LARGE_PAGES
        }
        match VirtualAlloc(addr, size, flags, PAGE_READWRITE) {
            NULL => None,
            addr => Some(addr as usize),
        }
    }
}
use self::paging::*;
use super::File;

pub const PAGE_EXEC:  u8 = 1 << 0;
pub const PAGE_READ:  u8 = 1 << 1;
pub const PAGE_WRITE: u8 = 1 << 2;

pub const PAGE_HUGE:        u8 = 1 << 3;
pub const PAGE_PRETOUCH:    u8 = 1 << 4;
pub const PAGE_MAPPED_RAW:  u8 = 1 << 5;
pub const PAGE_MAPPED_FILE: u8 = 1 << 6;

pub struct Page {
    flags: u8,
    pub size: usize,
    pub ptr: *mut u8,
}

impl Drop for Page {
    fn drop(&mut self) {
        if self.flags & PAGE_MAPPED_RAW == 0 {
            unsafe { page_free(self); }
        }
    }
}

impl Page {
    #[inline]
    pub fn as_raw(self) -> Page {
        Page {
            ptr: self.ptr,
            size: self.size,
            flags: self.flags | PAGE_MAPPED_RAW,
        }
    }

    #[inline]
    pub fn map(addr: usize, size: usize, flags: u8) -> Option<Page> {
        unsafe { page_map(addr, size, flags) }
    }

    #[inline]
    pub fn map_file(path: &str, size: usize, flags: u8) -> Option<File> {
        unsafe { page_map_file(path, size, flags) }
    }

    #[inline]
    pub fn map_from_file(file: &File, addr: usize, size: usize) -> Option<Page> {
        unsafe { page_map_from_file(file, addr, size) }
    }
}


#[cfg(linux)]
mod paging {
    use super::{*, super::{File, Handle}};

    const O_RDWR:  i32 = 2;
    const O_CREAT: i32 = 64;
    const O_EXCL:  i32 = 128;

    const PROT_READ:  i32 = 1;
    const PROT_WRITE: i32 = 2;
    const PROT_EXEC:  i32 = 4;

    const MAP_FAILED: usize = !0;
    const MAP_SHARED:    i32 = 1;
    const MAP_PRIVATE:   i32 = 2;
    const MAP_FIXED:     i32 = 16;
    const MAP_ANONYMOUS: i32 = 32;
    const MAP_HUGETLB:   i32 = 262144;
    const MAP_POPULATE:  i32 = 32768;

    #[link(name = "pthread")]
    extern "system" {
        fn ftruncate(fd: Handle, length: u32) -> i32;

        fn shm_unlink(name: *mut u8) -> i32;
        fn shm_open(name: *mut u8, flags: i32, mode: i32) -> Handle;

        fn munmap(addr: usize, size: usize) -> i32;
        fn mmap(addr: usize, size: usize, prot: i32, flags: i32, fd: Handle, offset: u32) -> usize;
    }

    #[inline]
    pub unsafe fn page_free(page: &Page) {

    }

    #[inline]
    pub unsafe fn page_map(addr: usize, size: usize, flags: u8) -> Option<Page> {
        
    }

    #[inline]
    pub unsafe fn page_map_file(path: &str, size: usize, flags: u8) -> Option<File> {
        
    }

    #[inline]
    pub unsafe fn page_map_from_file(file: &File, addr: usize, size: usize) -> Option<Page> {
        
    }
}

#[cfg(windows)]
mod paging {
    use super::{*, super::{File, Handle}};

    const SEC_RESERVE:     u32 = 0x4000000;
    const SEC_COMMIT:      u32 = 0x8000000;
    const SEC_LARGE_PAGES: u32 = 0x80000000;

    const PAGE_NOACCESS:          u32 = 0x01;
    const PAGE_READONLY:          u32 = 0x02;
    const PAGE_READWRITE:         u32 = 0x04;
    const PAGE_EXECUTE:           u32 = 0x10;
    const PAGE_EXECUTE_READ:      u32 = 0x20;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    const MEM_COMMIT:      u32 = 0x1000;
    const MEM_RESERVE:     u32 = 0x2000;
    const MEM_DECOMMIT:    u32 = 0x4000;
    const MEM_RELEASE:     u32 = 0x8000;
    const MEM_RESET:       u32 = 0x80000;
    const MEM_RESET_UNDO:  u32 = 0x1000000;
    const MEM_LARGE_PAGES: u32 = 0x20000000;

    extern "system" {
        fn VirtualFree(addr: usize, size: usize, free_type: u32) -> bool;
        fn VirtualAlloc(addr: usize, size: usize, mapping: u32, protect: u32) -> Handle;

        fn UnmapViewOfFile(addr: usize) -> bool;
        fn CreateFileMapping(_: Handle, _: usize, protect: u32, size_high: u32, size_low: u32, name: usize) -> Handle;
        fn MapViewOfFileEx(file: Handle, access: u32, offset_high: u32, offset_low: u32, size: usize, addr: usize) -> Handle;
    }

    #[inline]
    fn parse_flags(flags: u8) -> (u32, u32) {
        let mut mapping = 0;
        if flags & PAGE_PRETOUCH != 0 { mapping |= MEM_COMMIT }
        if flags & PAGE_HUGE != 0 { mapping |= MEM_LARGE_PAGES }
        
        const PROTECT_FLAGS: u8 = PAGE_EXEC | PAGE_READ | PAGE_WRITE;
        let protect = match flags & PROTECT_FLAGS {
            PAGE_EXEC => PAGE_EXECUTE,
            PAGE_READ => PAGE_READONLY,
            f if f == PAGE_READ | PAGE_EXEC => PAGE_EXECUTE_READ,
            f if f == PAGE_READ | PAGE_WRITE || f == PAGE_WRITE => PAGE_READWRITE,
            f if f == PAGE_EXEC | PAGE_WRITE || f == PROTECT_FLAGS => PAGE_EXECUTE_READWRITE,
            _ => PAGE_NOACCESS,
        };
        
        (mapping, protect)
    }

    #[inline]
    pub unsafe fn page_free(page: &Page) {
        if page.flags & PAGE_MAPPED_FILE != 0 {
            UnmapViewOfFile(page.ptr as usize);
        } else {
            VirtualFree(page.ptr as usize, 0, MEM_RELEASE);
        }
    }

    #[inline]
    pub unsafe fn page_map(addr: usize, size: usize, flags: u8) -> Option<Page> {
        let (mut mapping, protect) = parse_flags(flags);
        if mapping & MEM_COMMIT == 0 { mapping |= MEM_RESERVE }

        match VirtualAlloc(addr, size, protect, mapping) {
            0 => None,
            ptr => Some(Page {
                flags: 0,
                size: size,
                ptr: ptr as *mut u8
            })
        }
    }

    #[inline]
    pub unsafe fn page_map_file(_path: &str, size: usize, flags: u8) -> Option<File> {
        let (flags, _) = parse_flags(flags);
        let (high, low) = ((size >> 32) as u32, size as u32);

        let mut mapping = 0;
        if flags & MEM_LARGE_PAGES != 0 { mapping |= SEC_LARGE_PAGES }
        mapping |= if flags & MEM_COMMIT != 0 { SEC_COMMIT } else { SEC_RESERVE };

        match CreateFileMapping(File::invalid(), 0, mapping, high, low, 0) {
            0 => None,
            handle => Some(handle.into())
        }
    }

    #[inline]
    pub unsafe fn page_map_from_file(file: &File, addr: usize, size: usize) -> Option<Page> {
        const FILE_MAP_ALL_ACCESS: u32 = 0xF001F;

        match MapViewOfFileEx(file.handle, FILE_MAP_ALL_ACCESS, 0, 0, size, addr) {
            0 => None,
            ptr => Some(Page {
                size: size,
                ptr: ptr as *mut u8,
                flags: PAGE_MAPPED_FILE,
            })
        }
    }
}
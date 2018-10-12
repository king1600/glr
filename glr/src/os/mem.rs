pub const PAGE_EXEC:   i32 = 1 << 0;
pub const PAGE_READ:   i32 = 1 << 1;
pub const PAGE_WRITE:  i32 = 1 << 2;
pub const PAGE_HUGE:   i32 = 1 << 3;
pub const PAGE_COMMIT: i32 = 1 << 4;

pub struct PhysicalPage {
    #[cfg(unix)] handle: i32,
    #[cfg(windows)] handle: usize,
}

pub struct Page {
    addr: usize,
    size: usize,
    #[cfg(windows)] mapped: bool,
}

impl Drop for PhysicalPage {
    fn drop(&mut self) {
        unsafe { page_impl::physical_drop(self); }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe { page_impl::virtual_drop(self); }
    }
}

impl PhysicalPage {
    #[inline]
    pub fn new(size: usize, flags: i32) -> Self {
        unsafe { page_impl::physical_alloc(size, flags) }
    }

    #[inline]
    pub fn map(addr: usize, size: usize, flags: i32) -> Page {
        unsafe { page_impl::physical_map(self, addr, size, flags) }
    }
}

impl Page {
    #[inline]
    pub fn new(addr: usize, size: usize, flags: i32) -> Page {
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
        unsafe { page_impl::virtual_release(self); }
    }
}

#[cfg(unix)]
mod page_impl {
    use super::{super::*, Page, PhysicalPage};
    
    static mut physical_id: usize = 0;

    pub unsafe fn physical_drop(page: &PhysicalPage) {
        close(page.handle);
    }

    pub unsafe fn physical_map(page: &PhysicalPage, addr: usize, size: usize, flags: i32) -> Page {
        
    }

    pub unsafe fn physical_alloc(size: usize, flags: i32) -> PhysicalPage {
        let mut path = [c_char; 128];
        let id = atomic_xadd_relaxed(&physical_id, 1);
        snprintf(path.as_mut_ptr(), path.len(), "/glr(%d,%lu)", getpid(), id);

        let fd = shm_open(path.as_mut_ptr(), O_RDWR | O_CREAT | O_EXCL, 0o600);
        assert_ne!(fd, -1, "Failed to allocate physical page");
    }

    pub unsafe fn virtual_drop(page: &Page) {

    }

    pub unsafe fn virtual_release(page: &Page) {

    }

    pub unsafe fn virtual_alloc(addr: usize, size: usize, flags: i32) -> Page {

    }
}

#[cfg(windows)]
mod page_impl {
    use super::{super::*, Page, PhysicalPage};

    pub unsafe fn physical_drop(page: &PhysicalPage) {

    }

    pub unsafe fn physical_map(page: &PhysicalPage, addr: usize, size: usize, flags: i32) -> Page {
        
    }

    pub unsafe fn physical_alloc(size: usize, flags: i32) -> PhysicalPage {

    }

    pub unsafe fn virtual_drop(page: &Page) {

    }

    pub unsafe fn virtual_release(page: &Page) {

    }

    pub unsafe fn virtual_alloc(addr: usize, size: usize, flags: i32) -> Page {

    }
}